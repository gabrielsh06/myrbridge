use serde::{Deserialize, Serialize};

// structure for the request

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

// structure for the response

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoide>,
}

#[derive(Deserialize)]
struct ChatChoide {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

fn main() {
    println!("Hello, world!");
}
