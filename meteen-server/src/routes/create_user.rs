use std::io::Write;

use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::trace;
use axum::{extract::State, response::IntoResponse, Json};
use color_eyre::Result;
use entity::{prelude::*, user};
use meteen_model::Database as MeteenVault;
use rand::random;
use sea_orm::entity::ActiveModelTrait;
use sea_orm::IntoActiveModel;
use serde::{Deserialize, Serialize};
use sha2::Digest;

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
        password_hash: hash_password(password, salt.clone()),
        password_salt: salt,
        vault_id: name.clone(),
    };
    
    let transaction = match conn.get_postgres_connection_pool().begin().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to begin transaction").into_response();
        },
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

    match vaults.save_vault(&name, &vault).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error saving vault: {}", e);
            // TODO: Handle this error
            let _ = transaction.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create vault").into_response()
        }
    };

    match transaction.commit().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to commit transaction: {}", e);
            // TODO: Handle this error
            let _ = vaults.delete_vault(&name).await;
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to finalize transaction").into_response()
        },
    };


    (StatusCode::OK, "OK").into_response()
}

fn hash_password(password: String, salt: String) -> Vec<u8> {
    let salted = format!("{}{}", password, salt);

    let mut hasher = sha2::Sha512::new();
    hasher.update(salted);
    let hashed = hasher.finalize().to_vec();

    return hashed;
}
