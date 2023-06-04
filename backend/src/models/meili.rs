use std::collections::HashSet;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::mongo::{Sponsor, SponsorFavour, SponsorField};

#[derive(Serialize, Deserialize, Debug)]
pub struct MeiliSponsor {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    pub tags: HashSet<String>,
    pub fields: Vec<MeiliSponsorField>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeiliSponsorField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeiliSponsorFavour {
    pub id: Uuid,
    pub sponsor_uid: Uuid,
    pub condition: String,
    pub completed: bool,
    #[serde(rename = "dueUntil")]
    pub due_until: chrono::DateTime<Utc>,
}

impl From<Sponsor> for MeiliSponsor {
    fn from(value: Sponsor) -> Self {
        MeiliSponsor {
            id: value.uid.into(),
            name: value.name,
            short_description: value.short_description,
            tags: value.tags,
            fields: value.fields.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<SponsorField> for MeiliSponsorField {
    fn from(value: SponsorField) -> Self {
        MeiliSponsorField {
            name: value.name,
            value: value.value,
        }
    }
}

impl From<SponsorFavour> for MeiliSponsorFavour {
    fn from(value: SponsorFavour) -> Self {
        Self {
            id: value.uid.into(),
            condition: value.condition,
            sponsor_uid: value.sponsor_uid.into(),
            completed: value.completed,
            due_until: value.due_until,
        }
    }
}

impl MeiliSponsorFavour {
    pub fn from_sponsor_vec(vec: &[SponsorFavour]) -> Vec<MeiliSponsorFavour> {
        vec
            .iter()
            .map(|favour| MeiliSponsorFavour::from(favour.clone()))
            .collect()
    }
}
