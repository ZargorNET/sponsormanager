use std::collections::HashSet;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::meili::MeiliSponsorFavour;
use crate::models::mongo::{Sponsor, SponsorFavour, SponsorField};

#[derive(Serialize, Deserialize, Debug)]
pub struct RestSponsor {
    pub uid: Option<Uuid>,
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub fields: Vec<RestSponsorField>,
    pub tags: HashSet<String>,
    pub favours: Vec<RestSponsorFavour>,
    #[serde(rename = "favoursCompleted")]
    pub favours_completed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestSponsorField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestSponsorFavour {
    pub uid: Option<Uuid>,
    #[serde(rename = "sponsorUid")]
    pub sponsor_uid: Option<Uuid>,
    pub condition: String,
    pub completed: bool,
    #[serde(rename = "dueUntil")]
    pub due_until: chrono::DateTime<Utc>,
}

impl From<Sponsor> for RestSponsor {
    fn from(value: Sponsor) -> Self {
        let favours_completed = Some(value.favours.iter().all(|f| f.completed));

        Self {
            uid: Some(value.uid.into()),
            name: value.name,
            short_description: value.short_description,
            image_url: value.image_url,
            fields: value.fields.into_iter().map(|f| f.into()).collect(),
            tags: value.tags,
            favours: value.favours.into_iter().map(|f| f.into()).collect(),
            favours_completed,
        }
    }
}

impl From<SponsorField> for RestSponsorField {
    fn from(value: SponsorField) -> Self {
        Self {
            name: value.name,
            value: value.value,
        }
    }
}

impl From<SponsorFavour> for RestSponsorFavour {
    fn from(value: SponsorFavour) -> Self {
        Self {
            uid: Some(value.uid.into()),
            sponsor_uid: Some(value.sponsor_uid.into()),
            condition: value.condition,
            completed: value.completed,
            due_until: value.due_until,
        }
    }
}

impl From<MeiliSponsorFavour> for RestSponsorFavour {
    fn from(value: MeiliSponsorFavour) -> Self {
        Self {
            uid: Some(value.id),
            sponsor_uid: Some(value.sponsor_uid),
            condition: value.condition,
            completed: value.completed,
            due_until: value.due_until,
        }
    }
}
