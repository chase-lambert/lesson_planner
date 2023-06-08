use super::*;
use crate::openai::*;

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

// This struct will be used to pass data to the Askama template
#[derive(serde::Serialize, Template)]
#[template(path = "lesson_builder.html")]
struct LessonBuilder {
    pub history: Vec<ChatMessage>,
    pub conversation_history_json: String,
}

pub async fn lesson_builder(form: Form<LessonBuilderForm>) -> impl IntoResponse {
    let mut history: Vec<ChatMessage> =
        serde_json::from_str(&form.conversation_history).unwrap_or_default();

    // Append the user's message to the history
    history.push(ChatMessage {
        role: "user".to_string(),
        content: form.message.clone(),
    });

    // Generate AI's response
    let ai_response = chat_api_call(&history)
        .await
        .expect("Failed to get AI response");

    // Append the AI's response to the history
    history.push(ChatMessage {
        role: "assistant".to_string(),
        content: ai_response,
    });

    // Prepare the context for the Askama template
    let context = LessonBuilder {
        history: history.clone(),
        conversation_history_json: serde_json::to_string(&history)
            .expect("Failed to serialize conversation history"),
    };

    HtmlTemplate(context)
}
