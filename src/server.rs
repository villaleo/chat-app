use std::sync::Arc;

use crate::hub::{ Hub, HubOptions };
use crate::proto::InputParcel;

use futures::{StreamExt, TryStreamExt};
use log::{error, info};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Duration;
use warp::ws::WebSocket;
use warp::Filter;
use crate::client::Client;

const ALIVE_INTERVAL_SECS: u64 = 5;

pub struct Server {
    port: u16,
    hub: Arc<Hub>,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            hub: Arc::new(Hub::new(HubOptions {
                alive_interval: Some(Duration::from_secs(ALIVE_INTERVAL_SECS)),
            })),
        }
    }

    pub async fn run(&self) {
        let (input_sender, input_receiver) = mpsc::unbounded_channel::<InputParcel>();
        let hub = self.hub.clone();

        let feed = warp::path("feed")
            .and(warp::ws())
            .and(warp::any().map(move || input_sender.clone()))
            .and(warp::any().map(move || hub.clone()))
            .map(move |ws: warp::ws::Ws, input_sender, hub| {
                // When a connection is established and upgraded to a WebSocket, delegate it to Self::process_client
                ws.on_upgrade(move |web_socket| async move {
                    tokio::spawn(Self::process_client(hub, web_socket, input_sender));
                })
            });

        let shutdown = async {
            tokio::signal::ctrl_c().await
                .expect("failed to install CTRL+C signal handler");
        };
        let (_, serving) = warp::serve(feed).bind_with_graceful_shutdown(([127, 0, 0, 1], self.port), shutdown);
        let running_hub = self.hub.run(input_receiver);

        tokio::select! {
            _ = serving => {},
            _ = running_hub => {},
        }
    }

    /// Describes the lifetime of a client.
    async fn process_client(hub: Arc<Hub>, web_socket: WebSocket, input_sender: UnboundedSender<InputParcel>) {
        let output_receiver = hub.subscribe();
        let (ws_sink, ws_stream) = web_socket.split();
        let client = Client::new();

        info!("Client {} connected", client.id);

        let reading = client.read_input(ws_stream).try_for_each(|input_parcel| async {
            input_sender.send(input_parcel).unwrap();
            Ok(())
        });

        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(rx.forward(ws_sink));
        let writing = client.write_output(output_receiver.into_stream()).try_for_each(|msg| async {
            tx.send(Ok(msg)).unwrap();
            Ok(())
        });

        if let Err(err) = tokio::select! {
            result = reading => result,
            result = writing => result,
        } {
            error!("Client connection error: {}", err);
        }

        hub.on_disconnect(client.id).await;
        info!("Client {} disconnected", client.id);
    }
}
