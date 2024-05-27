use dotenv::dotenv;
use std::io;

mod chatbot;

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file
    let mut prompt = String::new();
    loop {
        println!("\nEnter your prompt:");
        io::stdin().read_line(&mut prompt).expect("Failed to read line");
        match chatbot::create_chat_completion(prompt.trim().to_string()).await {
            Ok(response) => println!("\nOpenAI Response: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
