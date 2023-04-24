use axum::{routing::get, Router};
use std::net::SocketAddr;

mod websocket;
pub use websocket::*;

pub mod response;

pub mod permissions;

pub mod standby;

pub mod send_serialized;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let websocket_router = websocket_router().await?;

    let app = Router::new().route("/", get(root)).merge(websocket_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
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
