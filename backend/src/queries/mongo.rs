use std::time::Duration;

use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::options::{ClientOptions, ReplaceOptions};
use uuid::Uuid;

use crate::models::mongo::{Settings, Sponsor};

const DB_NAME: &str = "sponsormanager";

pub struct MongoQueries {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
    pub sponsor_collection: Collection<Sponsor>,
    pub settings_collection: Collection<Settings>
}

impl MongoQueries {
    pub async fn new(connection_uri: &str) -> anyhow::Result<Self> {
        let mut options = ClientOptions::parse(connection_uri).await?;
        options.connect_timeout = Some(Duration::from_secs(10));
        options.server_selection_timeout = Some(Duration::from_secs(5));

        let client = mongodb::Client::with_options(options)?;
        let db = client.database(DB_NAME);
        let sponsor_collection = db.collection("sponsors");
        let settings_collection = db.collection("settings");

        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        Ok(Self { client, db, sponsor_collection, settings_collection })
    }

    pub async fn insert(&self, sponsor: &Sponsor) -> anyhow::Result<()> {
        self.sponsor_collection.insert_one(sponsor, None).await?;
        Ok(())
    }

    pub async fn get(&self, uid: Uuid) -> anyhow::Result<Option<Sponsor>> {
        Ok(self.sponsor_collection.find_one(Some(doc! {"_id": uid}), None).await?)
    }

    pub async fn delete(&self, uid: &Uuid) -> anyhow::Result<()> {
        self.sponsor_collection.delete_one(doc! {"_id": uid}, None).await?;
        Ok(())
    }

    pub async fn update(&self, uid: &Uuid, sponsor: &Sponsor) -> anyhow::Result<()> {
        self.sponsor_collection.replace_one(doc! {"_id": uid}, sponsor, ReplaceOptions::builder().upsert(false).build()).await?;
        Ok(())
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<Sponsor>> {
        let mut cursor = self.sponsor_collection.find(None, None).await?;
        let mut sponsors = Vec::new();
        while let Some(sponsor) = cursor.next().await {
            sponsors.push(sponsor?);
        }
        Ok(sponsors)
    }

    pub async fn get_settings(&self) -> anyhow::Result<Settings> {
        Ok(self.settings_collection.find_one(None, None).await?.unwrap_or_default())
    }

    pub async fn update_settings(&self, settings: &Settings) -> anyhow::Result<()> {
        self.settings_collection.replace_one(doc! {}, settings, ReplaceOptions::builder().upsert(true).build()).await?;
        Ok(())
    }
}
