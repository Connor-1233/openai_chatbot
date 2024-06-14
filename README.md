# Chatbot Project

## Description

This project is a Rust-based chatbot that integrates with the OpenAI Chat API to provide responses to user inputs. It uses `reqwest` for HTTP requests and `serde` for JSON serialization and deserialization. The chatbot can handle various prompts and return appropriate responses from the OpenAI model.

### Features

- Connects to the OpenAI Chat API.
- Sends user inputs to the API and retrieves responses.
- Handles JSON serialization and deserialization.
- Utilizes environment variables for API key management.

### File Descriptions

1. **main.rs**:
   - Contains the main function, which serves as the entry point of the application.
   - Loads environment variables using `dotenv`.
   - Accepts user input in a loop and calls the chatbot function to get responses from the OpenAI API.

2. **chatbot.rs**:
   - Defines the data structures required for the request and response bodies when interacting with the OpenAI Chat API.
   - Implements the `create_chat_completion` function that makes an HTTP POST request to the OpenAI API and processes the response.

### How to Run

1. **Prerequisites**:
   - Ensure you have Rust installed on your machine. You can download it from [here](https://www.rust-lang.org/tools/install).
   - Set up an OpenAI API key and save it in a `.env` file in the project directory. The `.env` file should contain:
     ```sh
     OPENAI_API_KEY=your_openai_api_key
     ```

2. **Clone the repository**:
   ```sh
   git clone <(https://github.com/Connor-1233/openai_chatbot.git)l>
   cd <openai_chatbot>
