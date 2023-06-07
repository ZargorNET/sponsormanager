use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{AppResult, AppState};
use crate::auth::User;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct DeleteStruct {
    uid: Uuid,
}

pub async fn delete(state: State<AppState>, _user: User, Json(ds): Json<DeleteStruct>) -> AppResult {
    let uid = ds.uid;

    let Some(sponsor) = state.mongo.get(uid.into()).await? else {
        return Err(AppError::new(400, "sponsor not found"));
    };

    let favours = sponsor.favours.into_iter()
        .map(|f| f.uid.into()).collect::<Vec<uuid::Uuid>>();

    state.mongo.delete_logo(&uid.into()).await?;
    state.mongo.delete(&uid.into()).await?;
    state.meili.delete_sponsor(&uid).await?;
    state.meili.delete_favours(&favours).await?;

    Ok(Json(json!({})).into_response())
}
