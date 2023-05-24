use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::{AppResult, AppState};
use crate::auth::User;
use crate::models::rest::RestSponsor;

pub async fn get_all(state: State<AppState>, _user: User) -> AppResult {
    Ok(Json(json!(state.mongo.get_all().await?.into_iter().map(RestSponsor::from).collect::<Vec<_>>())).into_response())
}
