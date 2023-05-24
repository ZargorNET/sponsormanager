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

pub async fn get_sponsor(state: State<AppState>, _user: User, Path(sponsor_id): Path<String>) -> AppResult {
    let uid = Uuid::from_str(&sponsor_id)?;

    match state.mongo.get(uid).await? {
        None => Err(AppError(404, "sponsor not found".to_string())),
        Some(sponsor) => Ok(Json(json!(RestSponsor::from(sponsor))).into_response())
    }
}
