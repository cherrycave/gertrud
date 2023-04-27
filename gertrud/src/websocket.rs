use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::ws::{Message, WebSocket},
    Router,
};
use futures::stream::SplitSink;
use tokio::sync::Mutex;

use crate::{key_type::KeyType, state::BackendState};

mod websocket_handler;

pub mod messages;

pub struct WebsocketConnection {
    pub addr: SocketAddr,
    pub key_type: KeyType,
    pub identifier: String,
    pub sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
}

pub fn websocket_router(state: BackendState) -> eyre::Result<Router> {
    Ok(Router::new()
        .route("/ws", axum::routing::get(websocket_handler::ws_handler))
        .with_state(state))
}
