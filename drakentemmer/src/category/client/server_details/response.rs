use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetailsResponse {
    pub attributes: ServerDetailsAttributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetailsAttributes {
    pub identifier: String,
    pub uuid: String,
    pub name: String,
    pub relationships: ServerDetailsRelationships,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetailsRelationships {
    pub allocations: ServerDetailsAllocations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetailsAllocations {
    pub data: Vec<ServerDetailsAllocationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetailsAllocationData {
    pub attributes: ServerDetailsAllocationAttributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetailsAllocationAttributes {
    pub ip: String,
    pub ip_alias: Option<String>,
    pub port: u16,
    pub is_default: bool,
}
