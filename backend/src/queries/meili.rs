use meilisearch_sdk::indexes::Index;
use uuid::Uuid;

use crate::models::meili::{MeiliSponsor, MeiliSponsorFavour};

const INDEX_SPONSORS: &str = "sponsors";
const INDEX_FAVOURS: &str = "favours";

pub struct MeiliQueries {
    pub client: meilisearch_sdk::Client,
    pub sponsor_index: Index,
    pub favours_index: Index,
}

impl MeiliQueries {
    pub async fn new(uri: &str, token: &str) -> anyhow::Result<Self> {
        let client = meilisearch_sdk::Client::new(uri, Some(token));

        client.create_index(INDEX_SPONSORS, Some("id")).await?;
        client.create_index(INDEX_FAVOURS, Some("id")).await?;

        let sponsor_index = client.index(INDEX_SPONSORS);
        let favours_index = client.index(INDEX_FAVOURS);

        Ok(Self {
            client,
            sponsor_index,
            favours_index,
        })
    }

    pub async fn insert_sponsor(&self, sponsor: &MeiliSponsor) -> anyhow::Result<()> {
        self.sponsor_index.add_documents(&[sponsor], Some("id")).await?;

        Ok(())
    }

    pub async fn insert_favours(&self, favours: &[MeiliSponsorFavour]) -> anyhow::Result<()> {
        self.favours_index.add_documents(favours, Some("id")).await?;

        Ok(())
    }

    pub async fn get_sponsors(&self, query: &str) -> anyhow::Result<Vec<MeiliSponsor>> {
        Ok(self.sponsor_index.search().with_query(query).execute::<MeiliSponsor>().await?.hits
            .into_iter().map(|s| s.result).collect())
    }

    pub async fn get_favours(&self, query: &str) -> anyhow::Result<Vec<MeiliSponsorFavour>> {
        Ok(self.favours_index.search().with_query(query).execute::<MeiliSponsorFavour>().await?.hits
            .into_iter().map(|s| s.result).collect())
    }

    pub async fn delete_sponsor(&self, uid: &Uuid) -> anyhow::Result<()> {
        self.sponsor_index.delete_document(uid).await?;

        Ok(())
    }

    pub async fn delete_favours(&self, uid: &[Uuid]) -> anyhow::Result<()> {
        self.favours_index.delete_documents(uid).await?;

        Ok(())
    }

    pub async fn get_all_sponsors(&self) -> anyhow::Result<Vec<MeiliSponsor>> {
        Ok(self.sponsor_index.get_documents::<MeiliSponsor>().await?.results)
    }

    pub async fn get_all_favours(&self) -> anyhow::Result<Vec<MeiliSponsorFavour>> {
        Ok(self.favours_index.get_documents::<MeiliSponsorFavour>().await?.results)
    }
}
