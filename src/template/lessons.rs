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
