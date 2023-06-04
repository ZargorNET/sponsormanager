use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::{AppResult, AppState};
use crate::auth::User;
use crate::error::AppError;
use crate::models::rest::{RestSponsor, RestSponsorFavour};

pub async fn search(state: State<AppState>, _user: User, query: Query<HashMap<String, String>>) -> AppResult {
    let Some(search) = query.get("search") else {
        return Err(AppError(400, "no search query".to_string()));
    };
    let Some(typ) = query.get("type") else {
        return Err(AppError(400, "no type query. must be sponsors or favours".to_string()));
    };

    let returns = match typ.to_lowercase().as_str() {
        "sponsors" => {
            json!(futures::future::try_join_all(state.meili.get_sponsors(search).await?
                .into_iter()
                .map(|x| state.mongo.get(x.id.into()))).await?
                .into_iter().flatten().map(RestSponsor::from).collect::<Vec<_>>())
        }
        "favours" => {
            json!(state.meili.get_favours(search).await?.into_iter().map(RestSponsorFavour::from).collect::<Vec<_>>())
        }
        _ => return Err(AppError(400, "invalid type query".to_string()))
    };

    Ok(Json(json!({"results": returns})).into_response())
}
