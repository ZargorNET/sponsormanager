use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::response::Response;
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::auth::{JwtInstance, LdapInstance};
use crate::error::AppError;
use crate::queries::meili::MeiliQueries;
use crate::queries::mongo::MongoQueries;

pub mod auth;
pub mod error;
pub mod models;
pub mod queries;
mod routes;
mod meili_sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy()
            .add_directive("ldap3=error".parse()?))
        .init();

    info!("Initializing state...");
    let state = Arc::new(AppStateStruct {
        meili: MeiliQueries::new("http://127.0.0.1:7700", "iO34H9ZObAWVobl8Q7krgbvNd-T2gweco-5sQlYW8h8").await?,
        mongo: MongoQueries::new("mongodb://root:verysafepassword@127.0.0.1:27017").await?,
        jwt: JwtInstance::new("very secret"),
        ldap: LdapInstance::new("ldap://ldap.forumsys.com:389", "cn=read-only-admin,dc=example,dc=com", "password", "dc=example,dc=com"),
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
        .route("/update_logo", post(routes::update_logo))
        .route("/settings/get", get(routes::settings::get))
        .route("/settings/update", post(routes::settings::update))
        .route("/login", post(routes::login))
        .layer(CorsLayer::new().allow_origin(cors::Any).allow_headers(cors::Any).allow_methods(cors::Any))
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(16 * 1024 * 1024));


    info!("Starting meili sync...");
    meili_sync::sync_meili(state.clone());

    info!("Testing ldap connection...");
    state.ldap.check_ldap_con().await?;

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
    jwt: JwtInstance,
    ldap: LdapInstance,
}
