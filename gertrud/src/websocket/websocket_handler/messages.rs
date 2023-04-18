use serde::{Deserialize, Serialize};

pub mod common;
pub mod incoming;
pub mod outgoing;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessage {
    pub message_id: String,
    pub payload: WebSocketMessagePayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "message_type")]
pub enum WebSocketMessagePayload {
    #[serde(rename = "ok")]
    GenericOk,
}
