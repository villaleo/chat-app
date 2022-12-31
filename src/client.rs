use std::{error, result};

use crate::error::{ Error, Result };
use crate::proto::{InputParcel, OutputParcel};

use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use uuid::Uuid;
use warp::filters::ws::WebSocket;
use warp::ws;

#[derive(Clone, Copy, Default)]
pub struct Client {
    pub id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn read_input(&self, stream: SplitStream<WebSocket>) -> impl Stream<Item = Result<InputParcel>> {
        let client_id = self.id;
        // Only accept text messages
        stream.take_while(|msg| {
            future::ready(
                if let Ok(message) = msg {
                    message.is_text()
                } else {
                    false
                }
            )
        })
        .map(move |msg: result::Result<ws::Message, warp::Error>| match msg {
            Err(err) => Err(Error::System(err.to_string())),
            Ok(msg) => {
                // Deserialize JSON into proto::Input
                let input = serde_json::from_str(msg.to_str().unwrap())?;
                Ok(InputParcel::new(client_id, input))
            },
        })
    }

    pub fn write_output<S, E>(&self, stream: S) -> impl Stream<Item = Result<warp::ws::Message>>
    where
        S: TryStream<Ok = OutputParcel, Error = E> + Stream<Item = result::Result<OutputParcel, E>>,
        E: error::Error,
    {
        todo!()
    }
}
