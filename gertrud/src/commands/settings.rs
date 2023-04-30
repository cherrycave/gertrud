use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::{doc, Document};

use crate::state::BackendState;

pub async fn post_settings(
    State(state): State<BackendState>,
    Path(id): Path<String>,
    Json(mut body): Json<Document>,
) -> Result<(), String> {
    let settings = state
        .settings_collection
        .find_one(doc! { "_id": id.clone() }, None)
        .await;

    if let Ok(Some(_)) = settings {
        let _ = state
            .settings_collection
            .delete_one(doc! { "_id": id.clone() }, None)
            .await;
    }

    body.insert("_id", id);

    let insert_result = state.settings_collection.insert_one(body, None).await;

    if let Err(e) = insert_result {
        tracing::error!("Could not insert settings: {}", e);
        return Err("Could not insert settings".to_string());
    }

    Ok(())
}

pub async fn get_settings(
    State(state): State<BackendState>,
    Path(id): Path<String>,
) -> Result<Json<Document>, String> {
    let settings = state
        .settings_collection
        .find_one(doc! { "_id": id }, None)
        .await;

    if let Err(e) = settings {
        tracing::error!("Could not get settings: {}", e);
        return Err("Could not get settings".to_string());
    }

    let settings = settings.unwrap();

    if let Some(settings) = settings {
        Ok(Json(settings))
    } else {
        Err("Settings not found".to_string())
    }
}
