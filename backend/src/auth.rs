use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::Serialize;

use crate::AppState;

#[derive(Serialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = ();

    async fn from_request_parts(_: &mut Parts, _: &AppState) -> Result<Self, Self::Rejection> {
        Ok(User {
            name: "Conner S".to_string(),
            email: "dwwddw@htw-berlin.de".to_string(),
        })
    }
}
