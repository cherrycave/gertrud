use std::{net::SocketAddr, ops::ControlFlow};

use axum::extract::ws::Message;

use super::messages::WebSocketMessage;

pub fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(text) => {
            let message: Result<WebSocketMessage, _> = serde_json::from_str(&text);
            if let Err(e) = message {
                tracing::error!("Could not parse message from {}: {}", who, e);
                return ControlFlow::Continue(());
            }

            let message = message.unwrap();

            tracing::debug!("{} sent message: {:?}", who, message);
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
                tracing::warn!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::debug!(">>> {} sent pong with {:?}", who, v);
        }
        Message::Ping(v) => {
            tracing::debug!(">>> {} sent ping with {:?}", who, v);
        }
        _ => {}
    }
    ControlFlow::Continue(())
}
