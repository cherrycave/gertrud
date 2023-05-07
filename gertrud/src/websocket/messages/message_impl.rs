use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericError {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRequest {
    pub players: Vec<String>,
    pub server: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterServerRequest {
    pub register: bool,
    pub server_type: String,
    pub identifier: String,
    pub host: String,
    pub port: u16,
}
