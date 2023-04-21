mod query;
mod template;

use template::*;

use axum::{
    // extract,
    // extract::Form,
    routing::get,
    Router,
};
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
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    let app = Router::new()
        .route("/", get(landing))
        .route("/query", get(show_form).post(post_query))
        .nest_service(
            "/static",
            ServeDir::new("static")
                .precompressed_gzip()
                .not_found_service(ServeFile::new("templates/landing.html")),
        );

    tracing::debug!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
