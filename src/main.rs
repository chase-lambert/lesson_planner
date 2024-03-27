mod db;
mod error;
mod handlers;
mod open_ai;
mod routes;
mod scratch;
mod types;

use std::sync::Arc;

pub use self::error::{MyError, MyResult};

use anyhow::{Context, Result};
use axum::{
    // extract::{Form, State},
    routing::get,
    Router,
};
use handlers::*;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use types::AppState;

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

    let shared_pool = Arc::new(pool);
    let app_state = AppState { db: shared_pool };

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u32>()
        .context("PORT must be a number")?;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("TcpListener creation failed")?;
    tracing::debug!("Listening on {port}");

    let app = Router::new()
        .route("/", get(public::landing))
        .merge(routes::public_routes())
        .merge(routes::authenticated_routes())
        .nest_service(
            "/static",
            ServeDir::new("static")
                .precompressed_gzip()
                .not_found_service(ServeFile::new("templates/sections/landing.html")),
        )
        .with_state(app_state);
    // .layer(tower_livereload::LiveReloadLayer::new());

    // TODO TESTING DB STUFF! MAKE SURE TO DELETE
    // scratch::create_dummy_user(&pool)
    //     .await
    //     .context("Failed to create dummy user")?;

    axum::serve(listener, app)
        .await
        .context("Unable to start axum service")?;

    Ok(())
}
