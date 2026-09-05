use serde::{Deserialize, Serialize};
use std::time::Duration;

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

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

pub fn send_prompt(host: &str, model: &str, prompt: &str) {
    let url = format!("{}/v1/chat/completions", host);

    let request_body = ChatRequest {
        model: model.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        temperature: 0.5,
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(240))
        .build()
        .unwrap();

    println!("Sending request to {} using model '{}'...", host, model);

    match client.post(&url).json(&request_body).send() {
        Ok(res) => match res.json::<ChatResponse>() {
            Ok(data) => {
                if let Some(choice) = data.choices.first() {
                    println!("\nResponse:\n{}", choice.message.content);
                }
            }
            Err(err) => println!("Error parsing JSON response: {}", err),
        },
        Err(err) => {
            println!("Connection error with {}: {}", host, err);
        }
    }
}
