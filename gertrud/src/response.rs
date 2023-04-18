use std::fmt::Debug;

use axum::Json;
use serde::{Deserialize, Serialize};

pub type CherryCaveResult = Result<Json<CherryCaveResponse>, Json<CherryCaveError>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CherryCaveError {
    pub message: String,
    pub display_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum CherryCaveResponse {
    #[serde(rename = "ok")]
    GenericOk,
}
