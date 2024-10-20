use axum::{
    routing::{get, post},
    Router,
};
use color_eyre::eyre::Result;
use sea_orm::{prelude::*, Database};

mod cfg;
mod routes;
mod vaults;

use routes::create_user::create_user;
use cfg::Config;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let config = Config::from_env()?;
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

    let listener = tokio::net::TcpListener::bind((address, port)).await?;

    let conn_str = format!("postgres://{db_user}:{db_pass}@{db_host}/{db_name}");
    dbg!(&conn_str);
    let connection = Database::connect(&conn_str).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/create", post(create_user))
        .with_state(AppState { conn: connection });

    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, Axum!"
}
