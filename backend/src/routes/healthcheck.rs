use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::AppResult;

pub async fn healthcheck() -> AppResult {
    Ok(Json(json!({"success": true})).into_response())
}
