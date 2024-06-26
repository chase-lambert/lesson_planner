use crate::{
    db::{create_user, get_user_by_email, verify_password},
    types::{AppState, NewUser},
};
use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::json;
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

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login_attempt(
    State(state): State<AppState>,
    Form(login_form): Form<LoginForm>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match get_user_by_email(&state.db, &login_form.email).await {
        Ok(user) => {
            if verify_password(&login_form.password, &user.hashed_password) {
                Ok(Json(json!({"redirect": "/lesson"})))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid email or password"))
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid email or password")),
    }
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
    Json(Json<serde_json::Value>),
}

impl IntoResponse for SignupResponse {
    fn into_response(self) -> Response {
        match self {
            SignupResponse::Template(t) => t.into_response(),
            SignupResponse::Json(j) => j.into_response(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SignupFormData {
    #[serde(flatten)]
    pub user: NewUser,
    pub confirm_password: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Form(form_data): Form<SignupFormData>,
) -> Result<SignupResponse, Infallible> {
    tracing::debug!("creating user with form data: {:?}", form_data.user.email);
    match create_user(&state.db, form_data.user).await {
        Ok(_) => {
            tracing::debug!("user created!");
            Ok(SignupResponse::Json(Json(json!({"redirect": "/lesson"}))))
        }
        Err(_) => {
            let template = SignupTemplate;
            Ok(SignupResponse::Template(HtmlTemplate(template)))
        }
    }
}
