use std::sync::Arc;

use axum::{
    async_trait,
    extract::ws::{Message, WebSocket},
};
use futures::{stream::SplitSink, SinkExt};
use tokio::sync::Mutex;

use crate::{messages::WebSocketMessage, WebsocketConnection};

#[async_trait]
pub trait SendSerialized {
    async fn send_serialized(&self, message: WebSocketMessage) -> eyre::Result<()>;
}

#[async_trait]
impl SendSerialized for WebsocketConnection {
    async fn send_serialized(&self, message: WebSocketMessage) -> eyre::Result<()> {
        self.sender.send_serialized(message).await
    }
}

#[async_trait]
impl SendSerialized for Arc<Mutex<SplitSink<WebSocket, Message>>> {
    async fn send_serialized(&self, message: WebSocketMessage) -> eyre::Result<()> {
        self.lock()
            .await
            .send(Message::Text(serde_json::to_string(&message)?))
            .await?;

        Ok(())
    }
}
