use std::{net::SocketAddr, ops::ControlFlow, sync::Arc};

use axum::extract::ws::Message;
use tokio::sync::Mutex;

use crate::{
    messages::WebSocketMessage, send_serialized::SendSerialized, standby::Standby,
    WebsocketConnection,
};

use self::send_request::process_send_request;

mod send_request;

pub fn process_message(
    msg: Message,
    who: SocketAddr,
    connections: Arc<Mutex<Vec<WebsocketConnection>>>,
    standby: Arc<Standby>,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(text) => {
            let message: Result<WebSocketMessage, _> = serde_json::from_str(&text);
            if let Err(e) = message {
                tracing::error!("Could not parse message from {}: {}", who, e);
                return ControlFlow::Continue(());
            }

            let message = message.unwrap();

            tracing::debug!("{} sent message: {:?}", who, message);

            match message.message_type {
                crate::messages::MessageType::Init => {
                    tokio::spawn(async move {
                        let result =
                            handle_init_message(who, message.clone(), connections.clone(), standby)
                                .await;

                        match result {
                            Ok(_) => {
                                let connections = connections.lock().await;
                                let connection = connections
                                    .iter()
                                    .find(|connection| connection.addr == who)
                                    .unwrap();

                                let _ = connection
                                    .send_serialized(WebSocketMessage {
                                        message_id: message.message_id,
                                        message_type: crate::messages::MessageType::Response,
                                        payload:
                                            crate::messages::WebSocketMessagePayload::GenericOk,
                                    })
                                    .await;
                            }
                            Err(err) => {
                                let connections = connections.lock().await;
                                let connection = connections
                                    .iter()
                                    .find(|connection| connection.addr == who)
                                    .unwrap();

                                let _ = connection
                                    .send_serialized(WebSocketMessage {
                                        message_id: message.message_id,
                                        message_type: crate::messages::MessageType::Response,
                                        payload:
                                            crate::messages::WebSocketMessagePayload::GenericError(
                                                crate::messages::common::GenericError {
                                                    message: err,
                                                },
                                            ),
                                    })
                                    .await;
                            }
                        }
                    });
                }
                crate::messages::MessageType::Response => {
                    standby.process_message(message);
                }
                crate::messages::MessageType::KeepAlive => {
                    standby.process_message(message);
                }
            }
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    "{} sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::warn!("{} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::debug!("{} sent pong with {:?}", who, v);
        }
        Message::Ping(v) => {
            tracing::debug!("{} sent ping with {:?}", who, v);
        }
        _ => {}
    }
    ControlFlow::Continue(())
}

#[allow(unused_variables)]
async fn handle_init_message(
    who: SocketAddr,
    message: WebSocketMessage,
    connections: Arc<Mutex<Vec<WebsocketConnection>>>,
    standby: Arc<Standby>,
) -> Result<(), String> {
    match message.payload.clone() {
        crate::messages::WebSocketMessagePayload::SendRequest(_) => {
            process_send_request(message.clone(), connections.clone(), standby).await
        }
        crate::messages::WebSocketMessagePayload::GenericOk => {
            Err("Unknown payload as init message".to_string())
        }
        crate::messages::WebSocketMessagePayload::GenericError(_) => {
            Err("Unknown payload as init message".to_string())
        }
        crate::messages::WebSocketMessagePayload::Empty => {
            Err("Unknown payload as init message".to_string())
        }
    }
}
