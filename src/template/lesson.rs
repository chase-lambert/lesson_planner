use super::*;

#[derive(Template)]
#[template(path = "sections/lessons.html")]
struct LessonsTemplate;

pub async fn lessons() -> impl IntoResponse {
    let template = LessonsTemplate;
    HtmlTemplate(template)
}

// #[derive(Template)]
// #[template(path = "form.html")]
// struct FormTemplate;

// pub async fn show_form() -> impl IntoResponse {
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

// pub async fn post_query(Form(input): Form<Prompt>) -> impl IntoResponse {
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
