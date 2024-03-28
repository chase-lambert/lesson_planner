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
#[template(path = "account.html")]
struct AccountTemplate;

pub async fn account() -> impl IntoResponse {
    let template = AccountTemplate;
    HtmlTemplate(template)
}
