use serde::{Deserialize, Serialize};

mod message_impl;
pub use message_impl::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessage {
    pub message_id: String,
    pub message_type: MessageType,
    pub payload: WebSocketMessagePayload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    Init,
    KeepAlive,
    Response,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "messageType")]
pub enum WebSocketMessagePayload {
    #[serde(rename = "ok")]
    GenericOk,
    #[serde(rename = "error")]
    GenericError(GenericError),
    #[serde(rename = "empty")]
    Empty,
    SendRequest(SendRequest),
    RegisterServerRequest(RegisterServerRequest),
}
