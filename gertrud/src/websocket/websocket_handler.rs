use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use redis::AsyncCommands;
use tokio::sync::Mutex;

use crate::{
    permissions::Permissions, websocket::websocket_handler::message_processor::process_message,
    WebsocketConnection, WebsocketState,
};

mod message_processor;

pub mod messages;

pub async fn ws_handler(
    State(state): State<WebsocketState>,
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("{addr} connected.");

    let auth_header = headers.get(AUTHORIZATION);

    if auth_header.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let auth_header = auth_header.unwrap();

    let auth_header = auth_header.to_str();

    if auth_header.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let auth_header = auth_header.unwrap();

    let mut redis = state.redis.lock().await;

    let permissions = redis.get(format!("auth.{}", auth_header)).await;

    drop(redis);

    if let Err(e) = permissions {
        tracing::error!("Could not get permissions for key {}: {}", auth_header, e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let permissions: Permissions = permissions.unwrap();

    Ok(ws.on_upgrade(move |socket| {
        handle_socket(socket, addr, state.connections.clone(), permissions)
    }))
}

async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    connections: Arc<Mutex<Vec<WebsocketConnection>>>,
    permissions: Permissions,
) {
    if socket.send(Message::Ping(vec![4, 2, 0])).await.is_ok() {
        tracing::debug!("Pinged new connection from {}...", who);
    } else {
        tracing::warn!("Could not send ping {}!", who);
        return;
    }

    let (sender, mut receiver) = socket.split();

    let sender = Arc::new(Mutex::new(sender));

    connections.lock().await.push(WebsocketConnection {
        permissions,
        sender: sender.clone(),
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg, who).is_break() {
                break;
            }
        }
    });

    let _ = recv_task.await;

    let _ = sender.lock().await.close().await;

    tracing::debug!("Websocket context {} destroyed", who);
}
