use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::{AppResult, AppState};
use crate::auth::User;

pub async fn changes(state: State<AppState>, _user: User, Path(offset): Path<String>) -> AppResult {
    let offset = offset.parse::<u64>()?.max(0);

    let (changes, total) = state.mongo.get_changes(offset).await?;

    Ok(Json(json!({"changes": changes, "total": total})).into_response())
}
