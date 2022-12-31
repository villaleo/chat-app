use std::{error, result};

use crate::error::{ Error, Result };
use crate::proto::{InputParcel, OutputParcel};

use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use uuid::Uuid;
use warp::filters::ws::WebSocket;

#[derive(Clone, Copy, Default)]
pub struct Client {
    pub id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn read_input(&self, stream: SplitStream<WebSocket>) -> impl Stream<Item = Result<InputParcel>> {
        todo!()
    }

    pub fn write_output<S, E>(&self, stream: S) -> impl Stream<Item = Result<warp::ws::Message>>
    where
        S: TryStream<Ok = OutputParcel, Error = E> + Stream<Item = result::Result<OutputParcel, E>>,
        E: error::Error,
    {
        todo!()
    }
}
