pub mod authenticated;
pub mod lesson;
pub mod public;

// pub use crate::{MyError, Result};
use crate::types::AppState;
pub use askama::Template;
pub use axum::{
    // extract,
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/landing", get(public::landing))
        .route("/demo", get(public::demo))
        .route("/signup", get(public::initial_signup).post(public::signup))
        .route("/login", get(public::login).post(public::login_attempt))
}

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/account", get(authenticated::account))
        .route("/classes", get(authenticated::classes))
        .route("/lesson", get(lesson::initial_lesson))
        .route("/lesson_builder", post(lesson::lesson_builder))
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate;

pub struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
