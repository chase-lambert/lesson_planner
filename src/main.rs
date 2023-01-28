mod query;

use askama::Template;
use axum::{
    // extract,
    // extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
// use query::run_query;
// use serde::Deserialize;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {addr}");

    let app = Router::new()
        .route("/", get(index))
        .merge(SpaRouter::new("/static", "static").index_file("templates/index.html"));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl IntoResponse {
    let template = IndexTemplate;
    HtmlTemplate(template)
}

// #[derive(Template)]
// #[template(path = "form.html")]
// struct FormTemplate;

// async fn show_form() -> impl IntoResponse {
//     let template = FormTemplate;
//     HtmlTemplate(template)
// }

// #[derive(Deserialize, Debug)]
// pub struct Prompt {
//     prompt: String,
// }

// #[derive(Deserialize, Debug, Default, Template)]
// #[template(path = "query.html")]
// #[allow(dead_code)]
// pub struct QueryTemplate {
//     prompt: String,
//     response: String,
// }

// async fn post_query(Form(input): Form<Prompt>) -> impl IntoResponse {
//     let response = run_query(&input.prompt).await;
//     let response = &response.unwrap().choices[0].text;

//     let template = QueryTemplate {
//         prompt: input.prompt,
//         response: response.to_owned(),
//     };

//     show_query(template).await
// }

// async fn show_query(query: QueryTemplate) -> impl IntoResponse {
//     HtmlTemplate(query)
// }

struct HtmlTemplate<T>(T);

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
