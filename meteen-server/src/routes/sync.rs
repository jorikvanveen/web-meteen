use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use entity::prelude::*;
use meteen_model::Operation;
use sea_orm::prelude::*;

use crate::{auth::check_auth_headers, AppState};

pub async fn sync(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(operations): Json<Vec<Operation>>,
) -> Result<Vec<u8>, Response> {
    // TODO: ACID transactions
    let AppState { conn, vaults } = state;

    let user = check_auth_headers(&conn, &headers).await?;
    let id = &user.username;

    let mut vaults = vaults.lock().await;
    {
        let vault = match vaults.get_vault_mut(id).await {
            Ok(vault) => vault,
            Err(e) => {
                eprintln!("Couldn't get vault: {}", e);
                return Err(
                    (StatusCode::NOT_FOUND, "No vault associated with user").into_response()
                );
            }
        };

        vault.batch_operations(operations);
    }

    vaults.save_cached_vault(id).await.map_err(|e| {
        eprintln!("Failed to save vault: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "failed to save vault").into_response()
    })?;

    match bincode::serialize(vaults.get_vault(id).await.unwrap()) {
        Ok(serialized) => Ok(serialized),
        Err(e) => {
            eprintln!("Error serializing vault: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Corrupted vault").into_response())
        }
    }
}
