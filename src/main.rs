use dotenv::dotenv;
use std::io;

mod chatbot;

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file
    let mut prompt = String::new();
    println!("Enter your prompt:");
    io::stdin().read_line(&mut prompt).expect("Failed to read line");
    // let prompt = "I've only gotten 6 hours of sleep over the last two days. What should I do?";

    match chatbot::create_chat_completion(prompt.trim().to_string()).await {
        Ok(response) => println!("OpenAI Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
