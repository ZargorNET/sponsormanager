use std::sync::Arc;

use axum::response::Response;
use axum::Router;
use axum::routing::{get, post};
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::error::AppError;
use crate::queries::meili::MeiliQueries;
use crate::queries::mongo::MongoQueries;

pub mod auth;
pub mod error;
pub mod models;
pub mod queries;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy())
        .init();

    info!("Initializing state...");
    let state = Arc::new(AppStateStruct {
        meili: MeiliQueries::new("http://127.0.0.1:7700", "iO34H9ZObAWVobl8Q7krgbvNd-T2gweco-5sQlYW8h8")?,
        mongo: MongoQueries::new("mongodb://root:verysafepassword@127.0.0.1:27017").await?,
    });


    let app = Router::new()
        .route("/", get(|| async { "Hello World. " }))
        .route("/health", get(routes::get_health))
        .route("/create", post(routes::create_sponsor));


    info!("Starting webserver...");
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app
            .with_state(state)
            .into_make_service())
        .await?;

    Ok(())
}

pub type AppResult = Result<Response, AppError>;
pub type AppState = Arc<AppStateStruct>;

pub struct AppStateStruct {
    meili: MeiliQueries,
    mongo: MongoQueries,
}
