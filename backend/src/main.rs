use std::process::exit;
use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::response::Response;
use axum::Router;
use axum::routing::{get, get_service, post};
use serde::Deserialize;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::auth::{JwtInstance, OpenIdInstance};
use crate::error::AppError;
use crate::queries::meili::MeiliQueries;
use crate::queries::mongo::MongoQueries;

pub mod auth;
pub mod error;
pub mod models;
pub mod queries;
pub mod misc;
mod routes;
mod meili_sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy()
            .add_directive("ldap3=error".parse()?))
        .init();

    info!("Fetching config...");
    let config = match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(e) => {
            error!("error fetching config: {:?}", e);
            exit(-1);
        }
    };

    info!("Initializing state...");
    let state = Arc::new(AppStateStruct {
        meili: MeiliQueries::new(&config.meili_uri, &config.meili_token).await?,
        mongo: MongoQueries::new(&config.mongo_url).await?,
        jwt: JwtInstance::new(&config.jwt_secret),
        oidc: OpenIdInstance::new(&config.oidc_client_id, &config.oidc_client_secret, &config.oidc_issuer_url, &config.hostname).await?,
    });


    let app = Router::new()
        .route("/", get(|| async { "Hello World. " }))
        .route("/health", get(routes::healthcheck))
        .route("/create", post(routes::create))
        .route("/search", get(routes::search))
        .route("/delete", post(routes::delete))
        .route("/whoami", get(routes::whoami))
        .route("/get/:sponsor_uid", get(routes::get))
        .route("/get_all", get(routes::get_all))
        .route("/get_logo/:sponsor_uid", get(routes::get_logo))
        .route("/update", post(routes::update))
        .route("/upload_logo", post(routes::upload_logo))
        .route("/settings/get", get(routes::settings::get))
        .route("/settings/update", post(routes::settings::update))
        .route("/login", get(routes::login))
        .route("/login/code", get(routes::login_code))
        .route("/changes/:offset", get(routes::changes))
        .layer(DefaultBodyLimit::max(16 * 1024 * 1024));

    let static_files_service = get_service(ServeDir::new("dist/")
        .append_index_html_on_directories(true)
        .fallback(ServeFile::new("dist/index.html")));

    info!("Starting meili sync...");
    meili_sync::sync_meili(state.clone());

    info!("Starting webserver {}...", &config.hostname);
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(Router::new()
            .fallback(static_files_service)
            .nest("/api", app)
            .layer(CorsLayer::new().allow_origin(cors::Any).allow_headers(cors::Any).allow_methods(cors::Any))
            .layer(TraceLayer::new_for_http())
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
    jwt: JwtInstance,
    oidc: OpenIdInstance,
}

#[derive(Deserialize, Debug)]
struct Config {
    meili_uri: String,
    meili_token: String,
    mongo_url: String,
    jwt_secret: String,
    hostname: String,

    oidc_client_id: String,
    oidc_client_secret: String,
    oidc_issuer_url: String,
}
