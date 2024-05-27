use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

//Define the structure of the request body for the OpenAI Chat API
#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

//Define the structure of the request body from the OpenAI Chat API
#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

//Define the structure of the response body from the OpenAI Chat API
#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

//Define the structure of the choice in the response body
#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

//Define the structure of the message in the choice
#[derive(Deserialize, Debug)]
struct Message {
    content: String,
}

pub async fn create_chat_completion(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    //Create a new reqwest client to make HTTP requests
    let client = Client::new();

    //Get the API Key From Environment
    let api_key = env::var("OPENAI_API_KEY")?;

    //Create a request body with the prompt 
    let request_body = ChatCompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        // model: "gpt-4o".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a sleep assistant, skilled in giving thorough explanations and responses to sleep questions.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    //Make a POST request to the OpenAI Chat API
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?
        .json::<ChatCompletionResponse>()
        .await?;
    
    //Return the response from the API or default message if no response
    Ok(response.choices.get(0).map_or("No response".to_string(), |choice| choice.message.content.clone()))
}
