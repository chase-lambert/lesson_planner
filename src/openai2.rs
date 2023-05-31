use crate::error::MyError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ChatRequest {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    role: String,
    content: String,
}

pub async fn get_response(messages: &[Message]) -> Result<Message, MyError> {
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

async fn foo() -> Result<(), MyError> {
    let starting_message = Message {
        role: "system".to_string(),
        content: "You are a helpful assistant".to_string(),
    };
    let mut messages: Vec<Message> = vec![starting_message];
    messages.push(get_response(&messages).await?);

    println!("\nWelcome to ChatGPT CLI");
    println!("Please enter a prompt");

    let prompt = "my temp prompt".to_string();

    let new_message = Message {
        role: "user".to_string(),
        content: prompt,
    };
    messages.push(new_message);

    let response = get_response(&messages).await?;
    println!("\nChatGPT response: {}", response.content);
    messages.push(response);

    Ok(())
}
