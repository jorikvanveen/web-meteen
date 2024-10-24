use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, Json};
use entity::user;
use meteen_model::Database as MeteenVault;
use sea_orm::entity::ActiveModelTrait;
use sea_orm::IntoActiveModel;
use serde::{Deserialize, Serialize};

use crate::auth::hash_password;
use crate::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateUser {
    name: String,
    password: String,
}

#[axum::debug_handler]
pub async fn create_user(state: State<AppState>, user: Json<CreateUser>) -> Response {
    let State(AppState { conn, vaults }) = state;
    let Json(CreateUser { name, password }) = user;

    let salt: String = nanoid::nanoid!();

    let user_model = user::Model {
        username: name.clone(),
        password_hash: hash_password(&password, &salt),
        password_salt: salt,
        vault_id: name.clone(),
    };

    let transaction = match conn.get_postgres_connection_pool().begin().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to begin transaction: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to begin transaction",
            )
                .into_response();
        }
    };

    match user_model.into_active_model().insert(&conn).await {
        Ok(model) => println!("Saved {:?}", model),
        Err(e) => {
            eprintln!("Failed to create user: {}", e);
            // TODO: Handle this error
            let _ = transaction.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response();
        }
    };

    let vault = MeteenVault::new();

    let vaults = vaults.lock().await;
    match vaults.save_vault(&name, &vault).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error saving vault: {}", e);
            // TODO: Handle this error
            let _ = transaction.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create vault").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to commit transaction: {}", e);
            // TODO: Handle this error
            let _ = vaults.delete_vault(&name).await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to finalize transaction",
            )
                .into_response();
        }
    };

    (StatusCode::OK, "OK").into_response()
}
