use ::mongodb::{options::ClientOptions, Client};
use axum::{routing::get, Router};
use drakentemmer::{Drakentemmer, GetClientApi};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

mod websocket;
pub use websocket::*;

use crate::{commands::commands_router, standby::Standby, state::BackendState};

mod commands;

pub mod response;

pub mod standby;

pub mod send_serialized;

pub mod key_type;

pub mod mongodb;

mod state;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set!");

    let mongo_client_options = ClientOptions::parse(mongo_url).await?;

    let mongo_client = Client::with_options(mongo_client_options)?;

    let pterodactyl_url = env::var("PTERODACTYL_URL").expect("PTERODACTYL_URL not set!");

    let client_token =
        env::var("PTERODACTYL_CLIENT_TOKEN").expect("PTERODACTYL_CLIENT_TOKEN not set!");

    let drakentemmer = Drakentemmer::new(pterodactyl_url, None, None);

    let gertrud_db = mongo_client.database("gertrud");

    let state = BackendState {
        redis: Arc::new(Mutex::new(
            redis::Client::open(env::var("REDIS_URL")?)?
                .get_tokio_connection_manager()
                .await?,
        )),
        connections: Arc::new(Mutex::new(Vec::new())),
        standby: Arc::new(Standby::default()),
        drakentemmer_client: Arc::new(drakentemmer.client(&client_token)),
        server_collection: gertrud_db.collection("registered_servers"),
        settings_collection: gertrud_db.collection("server_settings"),
    };

    let websocket_router = websocket_router(state.clone())?;
    let commands_router = commands_router(state.clone())?;

    let app = Router::new()
        .route("/", get(root))
        .merge(websocket_router)
        .nest("/commands/", commands_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], 6969));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Getrud hat bock!"
}
