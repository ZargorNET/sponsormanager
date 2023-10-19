use std::collections::HashSet;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

use crate::auth::{RequireAdmin, Role};
use crate::models::mongo::{Change, ChangeType, UserRole};
use crate::{AppResult, AppState};

#[derive(Deserialize)]
pub struct UpdateAdmins {
    admins: Vec<String>,
}

pub async fn update_admins(
    state: State<AppState>,
    user: RequireAdmin,
    Json(body): Json<UpdateAdmins>,
) -> AppResult {
    let db_admins: HashSet<UserRole> = state
        .mongo
        .get_all_admins()
        .await?
        .into_iter()
        .filter(|r| r.role == Role::ADMIN)
        .collect();
    let req_admins: HashSet<UserRole> = body
        .admins
        .into_iter()
        .map(|email| UserRole {
            email,
            role: Role::ADMIN,
        })
        .collect();

    let to_add = req_admins.difference(&db_admins);
    let to_remove = db_admins.difference(&req_admins);

    for role in req_admins.symmetric_difference(&db_admins) {
        state
            .mongo
            .add_change(&Change::new(
                &user.email,
                ChangeType::ChangeUserRole(role.clone()),
            ))
            .await?;
    }

    for role in to_add {
        state.mongo.add_or_update_role(role).await?;
    }

    for role in to_remove {
        state
            .mongo
            .add_or_update_role(&UserRole {
                email: role.email.clone(),
                role: Role::USER,
            })
            .await?;
    }

    Ok(Json(json!({})).into_response())
}
