use axum::{
    routing::{get, post},
    Router,
};
use color_eyre::eyre::Result;
use sea_orm::{prelude::*, Database};

mod cfg;
mod routes;
mod vaults;
mod auth;

use cfg::Config;
use routes::{create_user::create_user, get_vault::get_vault};
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
    vaults: Arc<Mutex<vaults::Vaults>>
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let config = Config::from_env().await?;
    dbg!(&config);
    let Config {
        db_user,
        db_name,
        db_pass,
        db_host,
        address,
        port,
        data_dir,
    } = config;

    tokio::fs::create_dir_all(&data_dir).await?;

    let listener = tokio::net::TcpListener::bind((address, port)).await?;

    let conn_str = format!("postgres://{db_user}:{db_pass}@{db_host}/{db_name}");
    dbg!(&conn_str);
    let connection = Database::connect(&conn_str).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/create", post(create_user))
        .route("/get/:id", get(get_vault))
        .with_state(AppState { conn: connection, vaults: Arc::new(Mutex::new(vaults::Vaults::new(data_dir))) });

    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, Axum!"
}
