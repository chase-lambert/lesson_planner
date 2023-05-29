use super::*;
// use crate::query::run_query;

#[derive(Template)]
#[template(path = "classes.html")]
struct ClassesTemplate;

pub async fn classes() -> impl IntoResponse {
    let template = ClassesTemplate;
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate;

pub async fn profile() -> impl IntoResponse {
    let template = ProfileTemplate;
    HtmlTemplate(template)
}
