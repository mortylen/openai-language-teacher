use std::io;
use serde::{Deserialize, Serialize};
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use reqwest::{StatusCode};

const MAX_MESSAGE_BUFFER: usize = 11;
const OPENAI_MODEL_URL: &'static str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTRequest {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub messages: Vec<Message>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct GPTResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
   pub  usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTError {
    pub error: ErrorContent
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorContent {
    pub messsge: String,
    pub err_type: String,
    pub param: String,
    pub code: String,
}

impl GPTRequest {
    pub fn new(model: String, temperature: f32, max_tokens: i32) -> Self {
        GPTRequest {
            model,
            temperature,
            max_tokens,
            messages: Vec::new(),
        }
    }
    
    pub fn add_message(&mut self, msg: Message) {
        if self.messages.len() >= MAX_MESSAGE_BUFFER {
            self.messages.drain(0..2);
        }
        self.messages.push(msg);
    }

    //TODO: remove and use add_message
    // pub fn add_system_message(&mut self, system_message: &String) {
    //     self.messages.push(Message{role: "system".to_string(), content: system_message.to_string()});
    // }

    pub fn remove_system_message(&mut self) {
        if self.messages.len() >= 1 {
            match self.messages.iter().position(|i| i.role == "system") {
                Some(msg_index) => {
                    self.messages.remove(msg_index);
                },
                None => {},
            }
        }
    }
}

pub async fn send_request(request: &GPTRequest, ywt_api_key: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post(OPENAI_MODEL_URL)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", ywt_api_key.trim()))
        .json(request)
        .send()
        .await
}

pub fn create_response_error<T>(error: T) -> GPTResponse where T: std::fmt::Display, {
    GPTResponse {
        choices: vec![Choice {
            index: 0,
            message: Message {
                role: "system".to_string(),
                content: error.to_string(),
            },
            finish_reason: "error".to_string(),
        }],
        ..Default::default()
    }
}

pub async fn parse_response(response: reqwest::Response) -> Result<GPTResponse, reqwest::Error> {
    match response.status() {
        StatusCode::OK => {
            let response_body = response.text().await?;
            let parse_result: Result<GPTResponse, _> = serde_json::from_str(&response_body);
            match parse_result {
                Ok(parsed_data) => {
                    Ok(parsed_data)
                },
                Err(error) => {
                    Ok(create_response_error(&error))
                },
            }
        }
        _ => {
            Ok(create_response_error(&response.text().await?))
        }
    }
}

pub async fn send_message(request: &GPTRequest, ywt_api_key: &str) -> Result<GPTResponse, reqwest::Error> {
    let response = send_request(request, ywt_api_key).await?;
    let parsed_data = parse_response(response).await?;
    Ok(parsed_data)
}

pub fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}