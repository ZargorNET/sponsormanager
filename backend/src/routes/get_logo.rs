use std::str::FromStr;

use axum::body::StreamBody;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{AppResult, AppState};
use crate::error::AppError;

pub async fn get_logo(state: State<AppState>, Path(sponsor_uid): Path<String>) -> AppResult {
    let uid = Uuid::from_str(&sponsor_uid)?;
    let Some(stream) = state.mongo.get_logo(&uid.into()).await?
        else { return Err(AppError::new(404, "logo not found")); };

    Ok(StreamBody::new(ReaderStream::new(stream)).into_response())
}
