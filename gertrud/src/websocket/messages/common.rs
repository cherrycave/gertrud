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
