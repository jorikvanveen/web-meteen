use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{auth::check_auth_headers, AppState};

pub async fn get_vault(
    //Path(id): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Vec<u8>, Response> {
    let AppState { vaults, conn } = state;

    let user = match check_auth_headers(&conn, &headers).await {
        Ok(user) => user,
        Err(r) => return Err(r),
    };

    let mut vaults = vaults.lock().await;
    let vault = vaults.get_vault(&user.username).await.map_err(|e| {
        eprintln!("Failed to get vault: {}", e);
        (StatusCode::NOT_FOUND, "Not found").into_response()
    })?;

    Ok(bincode::serialize(vault).unwrap().into())
}
