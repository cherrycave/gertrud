use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};
use redis::AsyncCommands;

use crate::{key_type::KeyType, state::BackendState};

use self::{
    send_request::send_request,
    server_registrations::{get_server_registrations, server_registrations},
    settings::{get_settings, post_settings},
};

mod send_request;
mod server_registrations;
mod settings;

#[derive(Clone)]
pub struct AuthorizationExtension {
    pub key_type: KeyType,
    pub identifier: String,
}

async fn auth<'c, B>(
    State(state): State<BackendState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let server_identifier = request
        .headers()
        .get("X-Server-Identifier")
        .and_then(|header| header.to_str().ok());

    let (auth_header, server_identifier) =
        if let (Some(auth_header), Some(server_identifier)) = (auth_header, server_identifier) {
            (auth_header.to_string(), server_identifier.to_string())
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        };

    let mut redis = state.redis.lock().await;

    let key_type = redis.get(format!("auth.{}", auth_header)).await;

    drop(redis);

    if let Err(e) = key_type {
        tracing::error!("Could not get server type from key {}: {}", auth_header, e);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let key_type = key_type.unwrap();

    request.extensions_mut().insert(AuthorizationExtension {
        key_type,
        identifier: server_identifier,
    });

    Ok(next.run(request).await)
}

pub fn commands_router(state: BackendState) -> eyre::Result<Router> {
    Ok(Router::new()
        .nest(
            "/servers/",
            Router::new()
                .route(
                    "/registrations",
                    post(server_registrations).get(get_server_registrations),
                )
                .route("/settings/:id", get(get_settings).post(post_settings))
                .route("/send", post(send_request)),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state))
}
