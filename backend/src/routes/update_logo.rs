use std::str::FromStr;

use axum::extract::{Multipart, State};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::{AppResult, AppState};
use crate::auth::User;
use crate::error::AppError;
use crate::models::mongo::{Change, ChangeType};
use crate::models::rest::RestSponsor;

pub async fn update_logo(state: State<AppState>, user: User, mut multipart: Multipart) -> AppResult {
    let mut sponsor_uid = None;
    let mut logo_data = None;

    while let Some(field) = multipart.next_field().await? {
        let Some(name) = field.name() else { return Err(AppError::new(400, "name field must be set")); };

        match name.to_lowercase().as_str() {
            "sponsor_uid" => sponsor_uid = Some(field.text().await?),
            "data" => {
                let Ok(bytes) = field.bytes().await else { return Err(AppError::new(400, "body too big (max 16MB)")); };
                logo_data = Some(bytes)
            }
            _ => return Err(AppError::new(400, "unknown field"))
        }
    }

    if sponsor_uid.is_none() || logo_data.is_none() {
        return Err(AppError::new(400, "missing data"));
    }
    let logo_data = logo_data.unwrap();

    let Some(mime) = infer::get(&logo_data) else { return Err(AppError::new(400, "invalid file type")); };
    match mime.mime_type() {
        "image/png" | "image/jpeg" => {},
        _ => return Err(AppError::new(400, "invalid file type"))
    }

    let sponsor_uid = uuid::Uuid::from_str(&sponsor_uid.unwrap())?;
    let mongo_uid = sponsor_uid.into();

    let Some(mut sponsor) = state.mongo.get(mongo_uid).await? else { return Err(AppError::new(400, "sponsor not found??")); };

    state.mongo.add_change(&Change::new(user.email, ChangeType::ChangeLogo(sponsor.clone()))).await?;

    state.mongo.upload_logo(&mongo_uid, logo_data).await?;
    sponsor.image_url = Some(format!("/get_logo/{}", &sponsor_uid.to_string()));
    state.mongo.update(&mongo_uid, &sponsor).await?;

    Ok(Json(json!(RestSponsor::from(sponsor))).into_response())
}
