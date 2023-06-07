use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::error;

pub struct AppError {
    status: u16,
    display: String,
    error: Option<anyhow::Error>,
}

impl AppError {
    pub fn new(status: u16, display: impl Into<String>) -> Self {
        Self {
            status,
            display: display.into(),
            error: None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap();
        if status.is_server_error() && self.error.is_some() {
            error!("error while serving request: {:?}", self.error);
        }

        (
            status,
            Json(json!({"error": &self.display}))
        ).into_response()
    }
}

impl<E> From<E> for AppError where E: Into<anyhow::Error> {
    fn from(err: E) -> Self {
        let error = err.into();
        AppError {
            status: 500,
            display: error.to_string(),
            error: Some(error),
        }
    }
}
