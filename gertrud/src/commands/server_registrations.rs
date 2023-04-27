use axum::{extract::State, response::Result, Extension, Json};
use drakentemmer::{client::server_details::ServerDetails, ClientApi};
use futures::StreamExt;
use mongodb::bson::doc;
use nanoid::nanoid;

use crate::{
    messages::{RegisterServerRequest, ServerType, WebSocketMessage},
    mongodb::RegisteredServer,
    send_serialized::SendSerialized,
    state::BackendState,
};

use super::AuthorizationExtension;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterServerBody {
    pub register: bool,
    pub server_type: ServerType,
}

pub async fn server_registrations(
    Extension(auth): Extension<AuthorizationExtension>,
    State(state): State<BackendState>,
    Json(body): Json<RegisterServerBody>,
) -> Result<(), String> {
    let result: Result<(String, u16), String> = if body.register {
        let (host, port) = get_from_ptero(&auth.identifier, &state.drakentemmer_client).await?;

        let _ = state
            .server_collection
            .delete_one(doc! { "_id": auth.identifier.clone() }, None)
            .await;

        let insert_result = state
            .server_collection
            .insert_one(
                RegisteredServer {
                    identifier: auth.identifier.clone(),
                    server_type: body.server_type.clone(),
                    host: host.clone(),
                    port,
                },
                None,
            )
            .await;

        if let Err(e) = insert_result {
            tracing::error!("Could not insert server: {}", e);
            return Err("Could not insert server".to_string());
        }

        Ok((host, port))
    } else {
        let database_server = state
            .server_collection
            .find_one(doc! { "_id": auth.identifier.clone() }, None)
            .await;

        if let Ok(Some(database_server)) = database_server {
            let host = database_server.host;
            let port = database_server.port;

            let delete_result = state
                .server_collection
                .delete_one(doc! { "_id": auth.identifier.clone() }, None)
                .await;

            if let Err(e) = delete_result {
                tracing::error!("Could not delete server: {}", e);
                return Err("Could not delete server".to_string());
            }

            Ok((host, port))
        } else {
            return Err("Server not found".to_string());
        }
    };

    let (host, port) = result?;

    let connections = state.connections.lock().await;

    for connection in connections.iter() {
        let sender = connection.sender.clone();
        let body = body.clone();
        let identifier = auth.identifier.clone();
        let host = host.clone();
        tokio::spawn(async move {
            let _ = sender
                .send_serialized(WebSocketMessage {
                    message_id: nanoid!(),
                    message_type: crate::messages::MessageType::Init,
                    payload: crate::messages::WebSocketMessagePayload::RegisterServerRequest(
                        RegisterServerRequest {
                            register: body.register,
                            server_type: body.server_type,
                            identifier,
                            host,
                            port,
                        },
                    ),
                })
                .await;
        });
    }

    Ok(())
}

pub async fn get_server_registrations(
    State(state): State<BackendState>,
) -> Result<Json<Vec<RegisterServerRequest>>, String> {
    let result = state.server_collection.find(doc! {}, None).await;

    if let Err(e) = result {
        tracing::error!("Could not get server registrations: {}", e);
        return Err("Could not get server registrations".to_string());
    }

    let mut result = result.unwrap();

    let mut registrations = Vec::new();

    while let Some(registration) = result.next().await {
        let registration = registration.map_err(|err| err.to_string())?;

        let registration = RegisterServerRequest {
            register: true,
            server_type: registration.server_type,
            identifier: registration.identifier,
            host: registration.host,
            port: registration.port,
        };

        registrations.push(registration);
    }

    Ok(Json(registrations))
}

async fn get_from_ptero(identifier: &str, client_api: &ClientApi) -> Result<(String, u16), String> {
    let server_details = client_api.get_server_details(identifier).await;

    if let Err(e) = server_details {
        tracing::error!("Could not get server details: {}", e);
        return Err("Could not get server details".to_string());
    }
    let server_details = server_details.unwrap();

    let default_allocation = server_details
        .attributes
        .relationships
        .allocations
        .data
        .iter()
        .find(|allocation| allocation.attributes.is_default)
        .unwrap_or_else(|| {
            server_details
                .attributes
                .relationships
                .allocations
                .data
                .first()
                .unwrap()
        });

    let host = default_allocation
        .attributes
        .ip_alias
        .as_ref()
        .unwrap_or(&default_allocation.attributes.ip);

    let port = default_allocation.attributes.port;

    Ok((host.clone(), port))
}
