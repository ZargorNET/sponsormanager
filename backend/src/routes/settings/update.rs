use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::{AppResult, AppState};
use crate::auth::User;
use crate::models::mongo::Settings;

pub async fn update(state: State<AppState>, _user: User, Json(settings): Json<Settings>) -> AppResult {
    state.mongo.update_settings(&settings).await?;

    Ok(Json(json!(settings)).into_response())
}
