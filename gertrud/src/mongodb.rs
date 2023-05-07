use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisteredServer {
    #[serde(rename = "_id")]
    pub identifier: String,
    pub server_type: String,
    pub host: String,
    pub port: u16,
}
