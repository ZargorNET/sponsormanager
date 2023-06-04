use std::collections::HashSet;

use chrono::Utc;
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sponsor {
    #[serde(rename = "_id")]
    pub uid: bson::Uuid,
    pub name: String,
    pub short_description: String,
    pub image_url: Option<String>,
    pub fields: Vec<SponsorField>,
    pub tags: HashSet<String>,
    pub favours: Vec<SponsorFavour>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SponsorField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SponsorFavour {
    #[serde(rename = "_id")]
    pub uid: bson::Uuid,
    pub sponsor_uid: bson::Uuid,
    pub condition: String,
    pub completed: bool,
    pub due_until: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Settings {
    #[serde(rename = "mandatoryFields")]
    pub mandatory_fields: HashSet<String>,
}
