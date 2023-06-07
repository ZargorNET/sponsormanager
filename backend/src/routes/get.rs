use std::str::FromStr;

use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use uuid::Uuid;

use crate::{AppResult, AppState};
use crate::auth::User;
use crate::error::AppError;
use crate::models::rest::RestSponsor;

pub async fn get(state: State<AppState>, _user: User, Path(sponsor_uid): Path<String>) -> AppResult {
    let uid = Uuid::from_str(&sponsor_uid)?;

    match state.mongo.get(uid.into()).await? {
        None => Err(AppError::new(404, "sponsor not found")),
        Some(sponsor) => Ok(Json(json!(RestSponsor::from(sponsor))).into_response())
    }
}
