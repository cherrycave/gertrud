use std::{borrow::Cow, net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use nanoid::nanoid;
use redis::AsyncCommands;
use tokio::{select, sync::Mutex};

use crate::{
    messages::WebSocketMessage, permissions::Permissions, standby::Standby,
    websocket::websocket_handler::message_processor::process_message, WebsocketConnection,
    WebsocketState,
};

mod message_processor;

pub async fn ws_handler(
    State(state): State<WebsocketState>,
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("{addr} connected.");

    let auth_header = headers.get(AUTHORIZATION);
    let server_identifier = headers.get("X-Server-Identifier");

    if auth_header.is_none() || server_identifier.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth_header = auth_header.unwrap().to_str();
    let server_identifier = server_identifier.unwrap().to_str();

    if auth_header.is_err() || server_identifier.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth_header = auth_header.unwrap();
    let server_identifier = server_identifier.unwrap().to_string();

    let mut redis = state.redis.lock().await;

    let permissions = redis.get(format!("auth.{}", auth_header)).await;

    drop(redis);

    if let Err(e) = permissions {
        tracing::error!("Could not get permissions for key {}: {}", auth_header, e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let permissions: Permissions = permissions.unwrap();

    Ok(ws.on_upgrade(move |socket| {
        handle_socket(
            socket,
            addr,
            server_identifier,
            state.connections.clone(),
            permissions,
            state.standby,
        )
    }))
}

async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    identifier: String,
    connections: Arc<Mutex<Vec<WebsocketConnection>>>,
    permissions: Permissions,
    standby: Arc<Standby>,
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
        addr: who,
        identifier: identifier.to_string(),
        permissions,
        sender: sender.clone(),
    });

    let connections_clone = connections.clone();
    let standby_clone = standby.clone();
    let recv_task = tokio::spawn(async move {
        let connections = connections_clone;
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg, who, connections.clone(), standby_clone.clone()).is_break() {
                break;
            }
        }
    });

    let sender_clone: Arc<Mutex<_>> = sender.clone();
    let keepalive_task = tokio::spawn(async move {
        let sender = sender_clone;
        loop {
            let id = nanoid!();
            let message = serde_json::to_string(&WebSocketMessage {
                message_id: id.clone(),
                message_type: crate::messages::MessageType::KeepAlive,
                payload: crate::messages::WebSocketMessagePayload::Empty,
            })
            .unwrap();
            let result = sender.lock().await.send(Message::Text(message)).await;

            if result.is_err() {
                tracing::warn!("Could not send keepalive to {}, disconnected!", who);
                break;
            }

            select! {
                _ = standby.clone().wait_for_response(id) => {
                    tracing::debug!("Keepalive response received from {}", who);
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    tracing::warn!("Keepalive response not received from {}!", who);
                    let _ = sender.lock().await.send(Message::Close(Some(CloseFrame {
                        code: 400,
                        reason: Cow::from("Keepalive response not received"),
                    }))).await;
                    break;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    select! {
        _ = keepalive_task => {
        },
        _ = recv_task => {
        },
    }

    let _ = sender.lock().await.close().await;

    connections.lock().await.retain(|c| c.addr != who);

    tracing::debug!("Websocket context {} destroyed", who);
}
