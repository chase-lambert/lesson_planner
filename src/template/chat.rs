use axum::debug_handler;
use reqwest::Client;

use crate::template::*;

#[derive(Deserialize)]
struct ChatRequest {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Clone, Deserialize, Template, Serialize)]
#[template(path = "lesson/chat.html")]
pub struct Message {
    role: String,
    content: String,
}

pub async fn get_response(messages: &[Message]) -> Result<Message> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("openai api key not found");
    let params = &serde_json::json!({"model": "gpt-3.5-turbo", "messages": messages});

    let request: ChatRequest = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .json(params)
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;

    let response = Message {
        role: "assistant".to_string(),
        content: request.choices.last().unwrap().message.content.to_string(),
    };

    Ok(response)
}

#[debug_handler]
pub async fn initial_chat() -> Result<impl IntoResponse> {
    let starting_message = Message {
        role: "system".to_string(),
        content:
            "You are a helpful teacher's assistant that builds thorough and detailed lesson plans"
                .to_string(),
    };

    let mut messages: Vec<Message> = vec![starting_message.clone()];
    messages.push(get_response(&messages).await?);

    Ok(HtmlTemplate(starting_message))
}

pub async fn chat(Form(input): Form<Message>) -> impl IntoResponse {
    let template = Message {
        role: input.content,
        content: "todo".to_string(),
    };

    HtmlTemplate(template)
}
