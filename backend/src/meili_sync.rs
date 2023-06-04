use std::panic::AssertUnwindSafe;
use std::process::exit;
use std::sync::Once;

use futures::FutureExt;
use tracing::{error, info};

use crate::AppState;
use crate::models::meili::MeiliSponsorFavour;
use crate::models::mongo::Sponsor;

static ALREADY_STARTED: Once = Once::new();

pub fn sync_meili(state: AppState) {
    if ALREADY_STARTED.is_completed() {
        return;
    }

    ALREADY_STARTED.call_once(|| {});
    tokio::spawn(async move {
        let future = AssertUnwindSafe(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(600));
            loop {
                interval.tick().await;

                if let Err(e) = run(&state).await {
                    error!("Failed to sync meili: {:?}", e);
                }
            }
        });

        if let Err(e) = future.catch_unwind().await {
            error!("PANIC while sync meili: {:?}", e);
            exit(1);
        }
    });
}

async fn run(state: &AppState) -> anyhow::Result<()> {
    info!("Syncing meili...");

    let mongo_sponsors = state.mongo.get_all().await?;

    let deleted = delete_dangling_meili(state, &mongo_sponsors).await?;
    let inserted = insert_all_to_meili(state, &mongo_sponsors).await?;

    info!("Suceessfully synced meili. Deleted: {}, Inserted: {}", deleted, inserted);

    Ok(())
}

async fn delete_dangling_meili(state: &AppState, mongo_docs: &[Sponsor]) -> anyhow::Result<usize> {
    let mut deleted = 0;
    for meili in state.meili.get_all_sponsors().await? {
        if mongo_docs.iter().any(|s| s.uid == meili.id.into()) {
            continue;
        }

        state.meili.delete_sponsor(&meili.id).await?;
        deleted += 1;
    }

    for meili in state.meili.get_all_favours().await? {
        if mongo_docs.iter().any(|s| s.favours.iter().any(|f| f.uid == meili.id.into())) {
            continue;
        }

        state.meili.delete_favours(&[meili.id]).await?;
    }


    Ok(deleted)
}

pub async fn insert_all_to_meili(state: &AppState, mongo_docs: &Vec<Sponsor>) -> anyhow::Result<usize> {
    let mut inserted = 0;
    for sponsor in mongo_docs {
        state.meili.insert_sponsor(&sponsor.clone().into()).await?;

        state.meili.insert_favours(
            &MeiliSponsorFavour::from_sponsor_vec(&sponsor.favours)
        ).await?;
        inserted += 1;
    }

    Ok(inserted)
}
