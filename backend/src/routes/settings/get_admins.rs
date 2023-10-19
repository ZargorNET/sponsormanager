use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::auth::RequireAdmin;
use crate::{AppResult, AppState};

pub async fn get_admins(state: State<AppState>, _user: RequireAdmin) -> AppResult {
    Ok(Json(json!(state
        .mongo
        .get_all_admins()
        .await?
        .into_iter()
        .map(|r| r.email)
        .collect::<Vec<String>>()))
    .into_response())
}
