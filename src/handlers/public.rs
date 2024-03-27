use crate::types::{AppState, NewUser};
use axum::extract::State;
use serde::Deserialize;

use super::*;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

pub async fn landing() -> impl IntoResponse {
    let template = LandingTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "demo.html")]
pub struct DemoTemplate;

pub async fn demo() -> impl IntoResponse {
    let template = DemoTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

pub async fn login() -> impl IntoResponse {
    let template = LoginTemplate;
    HtmlTemplate(template)
}

#[derive(Deserialize)]
struct SignupFormData {
    #[serde(flatten)]
    user: NewUser,
    confirm_password: String,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate;

pub async fn signup(State(state): State<AppState>) -> impl IntoResponse {
    let template = SignupTemplate;
    HtmlTemplate(template)
}
