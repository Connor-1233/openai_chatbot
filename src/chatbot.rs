use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Message {
    content: String,
}

pub async fn create_chat_completion(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY")?;

    let request_body = ChatCompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                // content: "You are a poetic assistant, skilled in explaining complex programming concepts with creative flair.".to_string(),
                content: "You are a sleep assistant, skilled in giving thorough explanations and responses to sleep questions.".to_string(),

            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?
        .json::<ChatCompletionResponse>()
        .await?;

    // println!("\n\nParsed response: {:?}\n\n\n", res);

    Ok(res.choices.get(0).map_or("No response".to_string(), |choice| choice.message.content.clone()))
}
