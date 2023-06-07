use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::{AppResult, AppState};
use crate::auth::User;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(state: State<AppState>, login_request: Json<LoginRequest>) -> AppResult {
    let Some(ldap_result) = state.ldap.search_user(&login_request.email).await? else { return Err(AppError::new(404, "user not found")); };
    if !state.ldap.check_password(&ldap_result, &login_request.password).await? { return Err(AppError::new(401, "invalid password")); };

    let user = User::from(ldap_result);
    let token = state.jwt.create_jwt(&user)?;


    Ok(Json(json!({"token": token})).into_response())
}
