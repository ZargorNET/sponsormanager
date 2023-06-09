use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::{AppResult, AppState};
use crate::auth::RequireAdmin;
use crate::models::mongo::{Change, ChangeType, Settings};

pub async fn update(state: State<AppState>, RequireAdmin(user): RequireAdmin, Json(settings): Json<Settings>) -> AppResult {
    state.mongo.add_change(&Change::new(user.email, ChangeType::ChangedSettings(settings.clone()))).await?;
    state.mongo.update_settings(&settings).await?;

    Ok(Json(json!(settings)).into_response())
}
