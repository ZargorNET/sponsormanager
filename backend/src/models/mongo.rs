use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sponsor {
    #[serde(rename = "_id", with = "mongodb::bson::serde_helpers::uuid_1_as_binary")]
    pub uid: Uuid,
    pub name: String,
    pub short_description: String,
    pub image_url: Option<String>,
    pub fields: Vec<SponsorField>,
    pub tags: Vec<String>,
    pub favours: Vec<SponsorFavour>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SponsorField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SponsorFavour {
    #[serde(rename = "_id", with = "mongodb::bson::serde_helpers::uuid_1_as_binary")]
    pub uid: Uuid,
    pub sponsor_uid: Uuid,
    pub condition: String,
    pub completed: bool,
    pub due_until: chrono::DateTime<Utc>,
}
