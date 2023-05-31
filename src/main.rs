mod error;
mod handlers;
mod openai;
mod openai2;
mod routes;

pub use self::error::{MyError, Result};

use axum::{
    // extract,
    // extract::Form,
    routing::get,
    Router,
};
use handlers::*;
use tower_http::services::{ServeDir, ServeFile};
// use query::run_query;
// use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lesson_planner=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse()
        .expect("PORT must be a number");
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

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

    tracing::debug!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
