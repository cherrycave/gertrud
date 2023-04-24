use axum::{async_trait, extract::ws::Message};
use futures::SinkExt;

use crate::{messages::WebSocketMessage, WebsocketConnection};

#[async_trait]
pub trait SendSerialized {
    async fn send_serialized(&self, message: WebSocketMessage) -> eyre::Result<()>;
}

#[async_trait]
impl SendSerialized for WebsocketConnection {
    async fn send_serialized(&self, message: WebSocketMessage) -> eyre::Result<()> {
        self.sender
            .lock()
            .await
            .send(Message::Text(serde_json::to_string(&message)?))
            .await?;

        Ok(())
    }
}
