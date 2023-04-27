use std::sync::Arc;

use drakentemmer::ClientApi;
use mongodb::Collection;
use redis::aio::ConnectionManager;
use tokio::sync::Mutex;

use crate::{mongodb::RegisteredServer, standby::Standby, WebsocketConnection};

#[derive(Clone)]
pub struct BackendState {
    pub redis: Arc<Mutex<ConnectionManager>>,
    pub connections: Arc<Mutex<Vec<WebsocketConnection>>>,
    pub standby: Arc<Standby>,
    pub drakentemmer_client: Arc<ClientApi>,
    pub server_collection: Collection<RegisteredServer>,
}
