use std::sync::Arc;

use chat_app::hub::{Hub, HubOptions};
use chat_app::proto::InputParcel;

use futures::{StreamExt, TryStreamExt};
use log::{error, info};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Duration;
use warp::ws::WebSocket;
use warp::Filter;

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
}
