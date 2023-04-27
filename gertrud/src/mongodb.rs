use serde::{Deserialize, Serialize};

use crate::messages::ServerType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisteredServer {
    #[serde(rename = "_id")]
    pub identifier: String,
    pub server_type: ServerType,
    pub host: String,
    pub port: u16,
}
