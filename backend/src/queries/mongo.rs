use std::time::Duration;

use axum::body::Bytes;
use futures::{AsyncWriteExt, StreamExt};
use mongodb::bson::doc;
use mongodb::options::{
    ClientOptions, FindOneAndReplaceOptions, FindOptions, GridFsBucketOptions, GridFsFindOptions,
    ReplaceOptions,
};
use mongodb::{bson, Collection, GridFsBucket, IndexModel};
use tokio_util::compat::FuturesAsyncReadCompatExt;

use crate::auth::Role;
use crate::models::mongo::{Change, Settings, Sponsor, UserRole};

const DB_NAME: &str = "sponsormanager";

pub struct MongoQueries {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
    pub sponsor_collection: Collection<Sponsor>,
    pub settings_collection: Collection<Settings>,
    pub change_collection: Collection<Change>,
    pub userrole_collection: Collection<UserRole>,
    pub logo_bucket: GridFsBucket,
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
        let change_collection = db.collection("changes");
        let userrole_collection = db.collection("userroles");
        let logo_bucket = db.gridfs_bucket(
            GridFsBucketOptions::builder()
                .bucket_name(Some("logos".to_string()))
                .build(),
        );

        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        userrole_collection
            .create_index(IndexModel::builder().keys(doc! {"email": 1}).build(), None)
            .await?;

        Ok(Self {
            client,
            db,
            sponsor_collection,
            settings_collection,
            change_collection,
            userrole_collection,
            logo_bucket,
        })
    }

    pub async fn insert(&self, sponsor: &Sponsor) -> anyhow::Result<()> {
        self.sponsor_collection.insert_one(sponsor, None).await?;
        Ok(())
    }

    pub async fn get(&self, uid: bson::Uuid) -> anyhow::Result<Option<Sponsor>> {
        Ok(self
            .sponsor_collection
            .find_one(Some(doc! {"_id": uid}), None)
            .await?)
    }

    pub async fn delete(&self, uid: &bson::Uuid) -> anyhow::Result<()> {
        self.sponsor_collection
            .delete_one(doc! {"_id": uid}, None)
            .await?;
        Ok(())
    }

    pub async fn update(&self, uid: &bson::Uuid, sponsor: &Sponsor) -> anyhow::Result<()> {
        self.sponsor_collection
            .replace_one(
                doc! {"_id": uid},
                sponsor,
                ReplaceOptions::builder().upsert(false).build(),
            )
            .await?;
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
        Ok(self
            .settings_collection
            .find_one(None, None)
            .await?
            .unwrap_or_default())
    }

    pub async fn update_settings(&self, settings: &Settings) -> anyhow::Result<()> {
        self.settings_collection
            .replace_one(
                doc! {},
                settings,
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(())
    }

    pub async fn upload_logo(&self, sponsor_uid: &bson::Uuid, file: Bytes) -> anyhow::Result<()> {
        let _ = self.delete_logo(sponsor_uid).await; // ignore errors

        let mut stream = self
            .logo_bucket
            .open_upload_stream(sponsor_uid.to_string(), None);
        stream.write_all(file.as_ref()).await?;
        stream.close().await?;

        Ok(())
    }

    pub async fn delete_logo(&self, sponsor_uid: &bson::Uuid) -> anyhow::Result<()> {
        let mut cursor = self
            .logo_bucket
            .find(
                doc! {"file_name": sponsor_uid.to_string()},
                GridFsFindOptions::builder().limit(1).build(),
            )
            .await?;
        let Some(find) = cursor.next().await else {
            return Ok(());
        };

        self.logo_bucket.delete(find?.id).await?;

        Ok(())
    }

    pub async fn get_logo(
        &self,
        sponsor_uid: &bson::Uuid,
    ) -> anyhow::Result<Option<impl tokio::io::AsyncRead>> {
        let stream = match self
            .logo_bucket
            .open_download_stream_by_name(sponsor_uid.to_string(), None)
            .await
        {
            Ok(s) => s.compat(),
            Err(e) => {
                if e.to_string().contains("FileNotFound") {
                    return Ok(None);
                }

                return Err(e.into());
            }
        };

        Ok(Some(stream))
    }

    pub async fn add_change(&self, change: &Change) -> anyhow::Result<()> {
        self.change_collection.insert_one(change, None).await?;

        Ok(())
    }

    pub async fn get_changes(&self, offset: u64) -> anyhow::Result<(Vec<Change>, u64)> {
        let mut cursor = self
            .change_collection
            .find(
                doc! {},
                FindOptions::builder()
                    .limit(100)
                    .skip(Some(offset))
                    .sort(doc! {"when": -1})
                    .build(),
            )
            .await?;

        let mut changes = Vec::new();
        while let Some(change) = cursor.next().await {
            changes.push(change?);
        }

        let total = self
            .change_collection
            .count_documents(doc! {}, None)
            .await?;
        Ok((changes, total))
    }

    pub async fn add_or_update_role(&self, model: &UserRole) -> anyhow::Result<()> {
        self.userrole_collection
            .find_one_and_replace(
                doc! {"email": &model.email},
                model,
                FindOneAndReplaceOptions::builder()
                    .upsert(Some(true))
                    .build(),
            )
            .await?;

        Ok(())
    }

    pub async fn get_user_role(&self, email: &str) -> anyhow::Result<Option<Role>> {
        let role = self
            .userrole_collection
            .find_one(doc! {"email": &email}, None)
            .await?;

        match role {
            None => Ok(None),
            Some(role) => Ok(Some(role.role)),
        }
    }

    pub async fn get_all_admins(&self) -> anyhow::Result<Vec<UserRole>> {
        let c = self
            .userrole_collection
            .find(doc! {"role": "ADMIN"}, None)
            .await?;
        let v = c
            .collect::<Vec<mongodb::error::Result<UserRole>>>()
            .await
            .into_iter()
            .collect::<mongodb::error::Result<Vec<UserRole>>>()?;

        Ok(v)
    }
}
