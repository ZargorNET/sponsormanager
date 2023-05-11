use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::AppResult;
use crate::auth::User;

pub async fn whoami(user: User) -> AppResult {
    Ok(Json(json!(user)).into_response())
}
