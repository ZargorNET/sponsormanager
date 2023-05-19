use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::error;

pub struct AppError(pub u16, pub String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.0).unwrap();
        if status.is_server_error() {
            error!("error while serving request: {}", self.1);
        }

        (
            status,
            Json(json!({"error": &self.1}))
        ).into_response()
    }
}

impl<E> From<E> for AppError where E: Into<anyhow::Error> {
    fn from(err: E) -> Self {
        AppError(500, err.into().to_string())
    }
}
