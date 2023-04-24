use serde::{Deserialize, Serialize};

use self::common::{GenericError, SendRequest};

pub mod common;
pub mod incoming;
pub mod outgoing;

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
}
