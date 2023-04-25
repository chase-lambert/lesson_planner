use super::*;

#[derive(Template)]
#[template(path = "auth/public.html")]
pub struct PublicTemplate;

pub async fn public() -> impl IntoResponse {
    let template = PublicTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "sections/landing.html")]
pub struct LandingTemplate;

pub async fn landing() -> impl IntoResponse {
    let template = LandingTemplate;
    HtmlTemplate(template)
}
