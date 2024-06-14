# Sleepscore Chatbot

Description
This project is a Rust-based chatbot that integrates with the OpenAI Chat API to provide responses to user inputs. It uses reqwest for HTTP requests and serde for JSON serialization and deserialization. The chatbot can handle various prompts and return appropriate responses from the OpenAI model.

Features
Connects to the OpenAI Chat API.
Sends user inputs to the API and retrieves responses.
Handles JSON serialization and deserialization.
Utilizes environment variables for API key management.
File Descriptions
main.rs:

Contains the main function, which serves as the entry point of the application.
Loads environment variables using dotenv.
Accepts user input in a loop and calls the chatbot function to get responses from the OpenAI API.
chatbot.rs:

Defines the data structures required for the request and response bodies when interacting with the OpenAI Chat API.
Implements the create_chat_completion function that makes an HTTP POST request to the OpenAI API and processes the response.
