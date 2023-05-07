use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::AppResult;

pub async fn get_health() -> AppResult {
    Ok(Json(json!({"success": true})).into_response())
}
