use super::*;

#[derive(Template)]
#[template(path = "auth/public.html")]
pub struct PublicTemplate;

// pub async fn public() -> impl IntoResponse {
//     let template = PublicTemplate;
//     HtmlTemplate(template)
// }

#[derive(Template)]
#[template(path = "sections/landing.html")]
pub struct LandingTemplate;

pub async fn landing() -> impl IntoResponse {
    let template = LandingTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "sections/demo.html")]
pub struct DemoTemplate;

pub async fn demo() -> impl IntoResponse {
    let template = DemoTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate;

pub async fn login() -> impl IntoResponse {
    let template = LoginTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "auth/signup.html")]
pub struct SignupTemplate;

pub async fn signup() -> impl IntoResponse {
    let template = SignupTemplate;
    HtmlTemplate(template)
}
