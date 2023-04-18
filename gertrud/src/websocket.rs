use std::{env, sync::Arc};

use axum::{
    extract::ws::{Message, WebSocket},
    Router,
};
use futures::stream::SplitSink;
use tokio::sync::Mutex;

use crate::permissions::Permissions;

mod websocket_handler;

#[derive(Clone)]
pub struct WebsocketState {
    pub redis: Arc<Mutex<redis::aio::ConnectionManager>>,
    pub connections: Arc<Mutex<Vec<WebsocketConnection>>>,
}

pub struct WebsocketConnection {
    pub permissions: Permissions,
    pub sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
}

pub async fn websocket_router() -> eyre::Result<Router> {
    let state = WebsocketState {
        redis: Arc::new(Mutex::new(
            redis::Client::open(env::var("REDIS_URL")?)?
                .get_tokio_connection_manager()
                .await?,
        )),
        connections: Arc::new(Mutex::new(Vec::new())),
    };

    Ok(Router::new()
        .route("/ws", axum::routing::get(websocket_handler::ws_handler))
        .with_state(state))
}
