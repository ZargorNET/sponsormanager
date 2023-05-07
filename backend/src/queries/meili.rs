use meilisearch_sdk::indexes::Index;

use crate::models::meili::{MeiliSponsor, MeiliSponsorFavour};

const INDEX_SPONSORS: &str = "sponsors";
const INDEX_FAVOURS: &str = "favours";

pub struct MeiliQueries {
    pub client: meilisearch_sdk::Client,
    pub sponsor_index: Index,
    pub favours_index: Index,
}

impl MeiliQueries {
    pub fn new(url: &str, token: &str) -> anyhow::Result<Self> {
        let client = meilisearch_sdk::Client::new(url, Some(token));

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
}
