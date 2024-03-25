mod db;
mod error;
mod handlers;
mod openai;
mod routes;
mod types;

pub use self::error::{MyError, MyResult};

use anyhow::{Context, Result};
use axum::{
    // extract,
    // extract::Form,
    routing::get,
    Router,
};
use handlers::*;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::{ServeDir, ServeFile};
// use query::run_query;
// use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lesson_planner=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL must be set correctly")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to the database")?;
    tracing::debug!("Connected to the database.");

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u32>()
        .context("PORT must be a number")?;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("TcpListener creation failed")?;
    tracing::debug!("listening on {port}");

    let app = Router::new()
        .route("/", get(public::landing))
        .merge(routes::public_routes())
        .merge(routes::authenticated_routes())
        .nest_service(
            "/static",
            ServeDir::new("static")
                .precompressed_gzip()
                .not_found_service(ServeFile::new("templates/sections/landing.html")),
        );
    // .layer(tower_livereload::LiveReloadLayer::new());

    axum::serve(listener, app)
        .await
        .context("Unable to start axum service")?;

    Ok(())
}
