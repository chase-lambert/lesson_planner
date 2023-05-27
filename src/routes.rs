use crate::template::{authenticated, lessons, public};

use axum::{
    routing::{get, post},
    Router,
};

pub fn public_routes() -> Router {
    Router::new()
        .route("/landing", get(public::landing))
        .route("/demo", get(public::demo))
        .route("/signup", get(public::signup))
        .route("/login", get(public::login))
}

pub fn authenticated_routes() -> Router {
    Router::new()
        .route("/classes", get(authenticated::classes))
        .route("/lesson", get(lessons::lesson_builder))
        .route("/lesson_viewer", post(lessons::lesson_viewer))
        .route("/profile", get(authenticated::profile))
}
