use dotenv::dotenv;

mod chatbot;

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file

    let prompt = "Compose a poem that explains the concept of recursion in programming.";
    match chatbot::create_chat_completion(prompt).await {
        Ok(response) => println!("OpenAI Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
