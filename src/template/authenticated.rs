use super::*;
// use crate::query::run_query;

#[derive(Template)]
#[template(path = "auth/authenticated.html")]
pub struct AuthenticatedTemplate;

// pub async fn authenticated() -> impl IntoResponse {
//     let template = AuthenticatedTemplate;
//     HtmlTemplate(template)
// }

#[derive(Template)]
#[template(path = "sections/classes.html")]
struct ClassesTemplate;

pub async fn classes() -> impl IntoResponse {
    let template = ClassesTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "sections/profile.html")]
struct ProfileTemplate;

pub async fn profile() -> impl IntoResponse {
    let template = ProfileTemplate;
    HtmlTemplate(template)
}
