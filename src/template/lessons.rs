use crate::query::run_query;
use crate::template::*;

#[derive(Template)]
#[template(path = "lesson/lesson_builder.html")]
struct LessonBuilder;

pub async fn lesson_builder() -> impl IntoResponse {
    let template = LessonBuilder;
    HtmlTemplate(template)
}

#[derive(Deserialize, Template)]
#[template(path = "lesson/lesson_viewer.html")]
struct LessonViewerTemplate {
    prompt: String,
    response: String,
}

#[derive(Deserialize, Debug)]
pub struct Prompt {
    prompt: String,
}

pub async fn lesson_viewer(Form(input): Form<Prompt>) -> impl IntoResponse {
    let response = run_query(&input.prompt).await;
    let response = &response.unwrap().choices[0].text;

    let template = LessonViewerTemplate {
        prompt: input.prompt,
        response: response.to_owned(),
    };

    show_query(template).await
}

async fn show_query(query: LessonViewerTemplate) -> impl IntoResponse {
    HtmlTemplate(query)
}

// #[derive(Template)]
// #[template(path = "form.html")]
// struct FormTemplate;

// pub async fn show_form() -> impl IntoResponse {
//     let template = FormTemplate;
//     HtmlTemplate(template)
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
