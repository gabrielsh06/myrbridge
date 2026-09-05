use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: myrbridge <prompt>");
        return;
    }

    let prompt = &args[1];

    let request_body = ChatRequest {
        model: "ornith-1.0-9b".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.clone(),
        }],
        temperature: 0.5,
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(240))
        .build()
        .unwrap();

    println!("Sending request...");

    let response = client
        .post("http://localhost:1234/v1/chat/completions")
        .json(&request_body)
        .send();

    match response {
        Ok(res) => match res.json::<ChatResponse>() {
            Ok(data) => {
                if let Some(choice) = data.choices.first() {
                    println!("Response:\n{}", choice.message.content);
                }
            }
            Err(err) => println!("Error parsing JSON response: {}", err),
        },
        Err(err) => {
            println!("Connection error: {}", err);
        }
    }
}
