use std::{net::SocketAddr, ops::ControlFlow, sync::Arc};

use axum::extract::ws::Message;

use crate::{messages::WebSocketMessage, standby::Standby};

pub fn process_message(
    msg: Message,
    who: SocketAddr,
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
                crate::messages::MessageType::Init => {}
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
