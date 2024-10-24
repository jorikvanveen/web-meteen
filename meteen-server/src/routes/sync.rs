use axum::{extract::State, http::HeaderMap, response::Response, Json};
use meteen_model::Operation;

use crate::{auth::check_auth_headers, AppState};

pub async fn sync(
    Json(operations): Json<Vec<Operation>>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(), Response> {
    let AppState { conn, vaults } = state;

    let user = check_auth_headers(&conn, &headers).await?;

    let vaults = vaults.lock().await;
    let vault = match vaults.get_vault().await {
        Ok(vault) => todo!(),
        Err(_) => todo!(),
    };
}
