use crate::{
    db::create_user,
    types::{AppState, NewUser},
};
use axum::extract::State;
use serde::Deserialize;
use std::convert::Infallible;

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

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate;

pub async fn initial_signup() -> impl IntoResponse {
    let template = SignupTemplate;
    HtmlTemplate(template)
}

pub enum SignupResponse {
    Template(HtmlTemplate<SignupTemplate>),
    Redirect(axum::response::Redirect),
}

impl IntoResponse for SignupResponse {
    fn into_response(self) -> Response {
        match self {
            SignupResponse::Template(t) => t.into_response(),
            SignupResponse::Redirect(r) => r.into_response(),
        }
    }
}

#[derive(Deserialize)]
pub struct SignupFormData {
    #[serde(flatten)]
    pub user: NewUser,
    _confirm_password: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Form(form_data): Form<SignupFormData>,
) -> Result<SignupResponse, Infallible> {
    match create_user(&state.db, form_data.user).await {
        Ok(_) => Ok(SignupResponse::Redirect(axum::response::Redirect::to(
            "/profile",
        ))),
        Err(_) => {
            let template = SignupTemplate;
            Ok(SignupResponse::Template(HtmlTemplate(template)))
        }
    }
}
