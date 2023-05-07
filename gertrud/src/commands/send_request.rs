use axum::{extract::State, Json};

use crate::{
    messages::{SendRequest, WebSocketMessage},
    send_serialized::SendSerialized,
    state::BackendState,
};
use nanoid::nanoid;

pub async fn send_request(
    State(state): State<BackendState>,
    Json(body): Json<SendRequest>,
) -> Result<(), String> {
    for connection in state.connections.lock().await.iter() {
        let sender = connection.sender.clone();
        let body = body.clone();
        tokio::spawn(async move {
            let _ = sender
                .send_serialized(WebSocketMessage {
                    message_id: nanoid!(),
                    message_type: crate::messages::MessageType::Init,
                    payload: crate::messages::WebSocketMessagePayload::SendRequest(body),
                })
                .await;
        });
    }

    Ok(())
}
