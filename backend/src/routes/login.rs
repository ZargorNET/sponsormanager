use axum::extract::State;
use axum::response::{IntoResponse, Redirect};

use crate::{AppResult, AppState};

pub async fn login(state: State<AppState>) -> AppResult {
    let redirect_uri = state.oidc.create_auth_url().await;

    Ok(Redirect::temporary(&redirect_uri).into_response())
}
