use std::sync::Arc;

use nanoid::nanoid;
use tokio::sync::Mutex;

use crate::{
    messages::WebSocketMessage, permissions::Permissions, send_serialized::SendSerialized,
    standby::Standby, WebsocketConnection,
};

pub async fn process_send_request(
    send_request: WebSocketMessage,
    connections: Arc<Mutex<Vec<WebsocketConnection>>>,
    standby: Arc<Standby>,
) -> Result<(), String> {
    let connections = connections.lock().await;

    let permitted_connections: Vec<&WebsocketConnection> = connections
        .iter()
        .filter(|c| c.permissions.contains(Permissions::READ_SEND) && c.identifier == "proxy")
        .collect();

    if permitted_connections.is_empty() {
        return Err("No eligable way to handle send request".to_string());
    }

    let connection = permitted_connections.first();

    let connection = match connection {
        Some(c) => c,
        None => return Err("No eligable way to handle send request".to_string()),
    };

    let new_id = nanoid!();

    let message = WebSocketMessage {
        message_id: new_id.clone(),
        message_type: crate::messages::MessageType::Init,
        payload: send_request.payload,
    };

    connection
        .send_serialized(message)
        .await
        .map_err(|e| e.to_string())?;

    drop(connections);

    let response = standby.wait_for_response(new_id).await;

    let response = match response {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    match response.payload {
        crate::messages::WebSocketMessagePayload::GenericOk => Ok(()),
        crate::messages::WebSocketMessagePayload::GenericError(err) => Err(err.message),
        _ => Err("Unexpected payload".to_string()),
    }
}
