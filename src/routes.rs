// use crate::handlers::{authenticated, lessons, public};
use crate::handlers::{authenticated, lesson, public};
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
        .route("/account", get(authenticated::account))
        .route("/classes", get(authenticated::classes))
        .route("/lesson", get(lesson::initial_lesson))
        .route("/lesson_builder", post(lesson::lesson_builder))
        .route("/profile", get(authenticated::profile))
}
