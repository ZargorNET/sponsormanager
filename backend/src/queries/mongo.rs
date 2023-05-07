use std::time::Duration;

use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::options::ClientOptions;

use crate::models::mongo::Sponsor;

const DB_NAME: &str = "sponsormanager";

pub struct MongoQueries {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
    pub sponsor_collection: Collection<Sponsor>,
}

impl MongoQueries {
    pub async fn new(connection_uri: &str) -> anyhow::Result<Self> {
        let mut options = ClientOptions::parse(connection_uri).await?;
        options.connect_timeout = Some(Duration::from_secs(10));
        options.server_selection_timeout = Some(Duration::from_secs(5));

        let client = mongodb::Client::with_options(options)?;
        let db = client.database(DB_NAME);
        let sponsor_collection = db.collection("sponsors");

        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        Ok(Self { client, db, sponsor_collection })
    }

    pub async fn insert(&self, sponsor: &Sponsor) -> anyhow::Result<()> {
        self.sponsor_collection.insert_one(sponsor, None).await?;
        Ok(())
    }
}
