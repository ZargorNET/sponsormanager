use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::{AppResult, AppState, misc};
use crate::auth::User;
use crate::error::AppError;
use crate::models::meili::MeiliSponsorFavour;
use crate::models::mongo::{Change, ChangeType, Sponsor, SponsorFavour, SponsorField};
use crate::models::rest::RestSponsor;

pub async fn create(state: State<AppState>, user: User, payload: Json<RestSponsor>) -> AppResult {
    let mut payload = payload.0;
    payload.uid = Some(Uuid::new_v4());

    let mongo_sponsor = Sponsor {
        uid: payload.uid.unwrap().into(),
        name: payload.name,
        short_description: payload.short_description,
        image_url: payload.image_url,
        fields: payload.fields.into_iter().map(|field| SponsorField { name: field.name, value: field.value }).collect(),
        tags: payload.tags,
        favours: payload.favours.into_iter().map(|favour| SponsorFavour {
            uid: Uuid::new_v4().into(),
            sponsor_uid: payload.uid.unwrap().into(),
            condition: favour.condition,
            completed: favour.completed,
            due_until: favour.due_until,
        }).collect(),
    };

    if let Err(e) = misc::assert_sponsor(&mongo_sponsor) {
        return Err(AppError::new(400, e.to_string()));
    }

    state.mongo.add_change(&Change::new(user.email, ChangeType::AddSponsor(mongo_sponsor.clone()))).await?;
    state.mongo.insert(&mongo_sponsor).await?;

    state.meili.insert_sponsor(&mongo_sponsor.clone().into()).await?;
    state.meili.insert_favours(
        &MeiliSponsorFavour::from_sponsor_vec(&mongo_sponsor.favours)
    ).await?;

    Ok(Json(RestSponsor::from(mongo_sponsor)).into_response())
}
