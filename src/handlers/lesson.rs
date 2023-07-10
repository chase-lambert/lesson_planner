use super::*;
use crate::openai::*;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Template)]
#[template(path = "initial_lesson.html")]
struct InitialLesson;

pub async fn initial_lesson() -> impl IntoResponse {
    let template = InitialLesson;
    HtmlTemplate(template)
}

#[derive(serde::Deserialize)]
pub struct LessonBuilderForm {
    pub message: String,
    pub conversation_history: String, // JSON-serialized conversation history
}

#[derive(serde::Serialize, Template)]
#[template(path = "lesson_builder.html")]
struct LessonBuilder {
    pub history: Vec<ChatMessage>,
    pub conversation_history_json: String,
}

#[derive(serde::Serialize, Template)]
#[template(path = "lesson_builder_inner.html")]
struct LessonBuilderInner {
    pub history: Vec<ChatMessage>,
    pub conversation_history_json: String,
}

pub async fn lesson_builder(form: Form<LessonBuilderForm>) -> impl IntoResponse {
    let mut history: Vec<ChatMessage> =
        serde_json::from_str(&form.conversation_history).unwrap_or_default();

    let user_prompt = form.message.clone();
    let user_prompt = format!("Please return all responses in Markdown format. {user_prompt}");
    // Append the user's message to the history
    history.push(ChatMessage {
        role: "user".to_string(),
        content: user_prompt,
    });

    // Generate AI's response
    let ai_response = chat_api_call(&history)
        .await
        .expect("Failed to get AI response");

    let ai_response_html = markdown_to_html(&ai_response, &ComrakOptions::default());

    println!("{ai_response}");

    // Append the AI's response to the history
    history.push(ChatMessage {
        role: "assistant".to_string(),
        content: ai_response_html,
    });

    // Prepare the context for the Askama template
    let context = LessonBuilderInner {
        history: history.clone(),
        conversation_history_json: serde_json::to_string(&history)
            .expect("Failed to serialize conversation history"),
    };

    HtmlTemplate(context)
}
