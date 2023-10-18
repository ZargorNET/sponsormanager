use axum::extract::{Query, State};
use axum::http::header::SET_COOKIE;
use axum::http::HeaderValue;
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;

use crate::auth::OpenIdInstance;
use crate::{AppResult, AppState};

#[derive(Deserialize)]
pub struct QueryParams {
    code: String,
    state: String,
}

pub async fn login_code(state: State<AppState>, query: Query<QueryParams>) -> AppResult {
    let claims = state
        .oidc
        .fetch_token(query.code.clone(), query.state.clone())
        .await?;
    let user = OpenIdInstance::user_from_claims(&state.mongo, claims).await?;

    let jwt = state.jwt.create_jwt(&user)?;

    let mut response = Redirect::temporary(&state.config.frontend_url).into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(
            format!("session={}; Secure; Path=/; Max-Age=2592000", &jwt).as_str(),
        )?,
    );

    Ok(response)
}
