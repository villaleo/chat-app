use std::{error, result};

use crate::error::{ Error, Result };
use crate::proto::{InputParcel, OutputParcel};

use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use uuid::Uuid;
use warp::{ filters::ws::WebSocket, ws::Message };

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
        .map(move |msg: result::Result<Message, warp::Error>| match msg {
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
        let client_id = self.id;
        stream.try_filter(move |output_parcel| {
            // Skip irrelevant parcels
            future::ready(output_parcel.client_id == client_id)
        })
        .map_ok(|output_parcel| {
            // Serialize parcel
            let data = serde_json::to_string(&output_parcel.output).unwrap();
            warp::ws::Message::text(data)
        })
        .map_err(|err| Error::System(err.to_string()))
    }
}
