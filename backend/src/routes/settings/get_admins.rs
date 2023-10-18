use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::auth::User;
use crate::{AppResult, AppState};

pub async fn get_admins(state: State<AppState>, _user: User) -> AppResult {
    Ok(Json(json!(state.mongo.get_all_admins().await?)).into_response())
}
