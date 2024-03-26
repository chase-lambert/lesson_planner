use crate::error::MyError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ChatRequest {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub async fn chat_api_call(history: &[ChatMessage]) -> Result<String, MyError> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("openai api key not found");
    let params = &serde_json::json!({"model": "gpt-3.5-turbo", "messages": history});

    let request: ChatRequest = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .json(params)
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;

    let ai_response = request.choices.last().unwrap().message.content.to_string();

    Ok(ai_response)
}
