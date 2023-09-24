use std::env;
use std::io;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value, json};
//use reqwest::{Client, Error, header};
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use reqwest::{Error, StatusCode};

const MAX_MESSAGE_BUFFER: usize = 10;
//const OPENAI_MODEL: &'static str = "gpt-4";
const OPENAI_MODEL: &'static str = "gpt-3.5-turbo";
const OPENAI_MODEL_URL: &'static str = "https://api.openai.com/v1/chat/completions";
const OPENAI_TEMPERATURE: f32 = 0.8;
const OPENAI_MAXTOKENS: i32 = 2048; 
const CONSOLE_GREEN_COLOR: &'static str = "\u{1b}[32m";
const CONSOLE_BLUE_COLOR: &'static str = "\u{1b}[34m";
const CONSOLE_RESET_COLOR: &'static str = "\u{1b}[39m";
const CONSOLE_BOLD_STYLE: &'static str = "\u{1b}[1m";
const CONSOLE_RESET_BOLD: &'static str = "\u{1b}[22m";

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GPTRequest {
    model: String,
    temperature: f32,
    max_tokens: i32,
    messages: Vec<Message>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct GPTResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    index: i32,
    message: Message,
    finish_reason: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GPTError {
    error: ErrorContent
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorContent {
    messsge: String,
    err_type: String,
    param: String,
    code: String,
}

impl GPTRequest {
    fn new(model: String, temperature: f32, max_tokens: i32) -> Self {
        GPTRequest {
            model,
            temperature,
            max_tokens,
            messages: Vec::new(),
        }
    }
    
    fn add_message(&mut self, msg: Message) {
        if self.messages.len() >= MAX_MESSAGE_BUFFER {
            self.messages.remove(1);
        }
        self.messages.push(msg);
    }
}

fn wait_for_api_ywt() -> String {
    let args: Vec<String> = env::args().collect();
    let api_ywt: String = if args.len() > 1 {
        args[1].to_string()
    } else {
        println!("Please enter your OpenAI API key (do not share your API key with others, or expose it in the browser or other client-side code): ");
        get_user_input()
    };

    api_ywt
}

fn set_language() -> String {
    println!("Please type the language you want to learn (for example 'English'): ");
    get_user_input()
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn set_openai_chat(language: &String) -> GPTRequest {
    let mut ai_chat = GPTRequest::new(OPENAI_MODEL.to_string(), OPENAI_TEMPERATURE, OPENAI_MAXTOKENS);
    let message = format!("You are my lector of {0} language.", language);
    ai_chat.add_message(Message{role: "system".to_string(), content: message});
    ai_chat
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ywt_api_key = wait_for_api_ywt();
    let language = set_language();
    let mut ai_chat = set_openai_chat(&language);

    println!("Please start the conversation: ");
    
    loop {
        println!("{0}{1}You: {2}{3}", CONSOLE_BLUE_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR); 
        let message: String = format!("I would like you to split each of your replies into two part. In the first part called as 'Correction:', correct and describe any mistakes in my {0} language. In the second part called as 'Conversation:', feel free to respond to my statement and continue the conversation. \n '{1}'", language,  get_user_input());
        ai_chat.add_message(Message{role: "user".to_string(), content: message});
    
        println!("\n");
        
        match send_message(&ai_chat, &ywt_api_key).await {
            Ok(response) => {
                let message_content = Message {
                    role: response.choices[0].message.role.clone(),
                    content: response.choices[0].message.content.clone(),
                };
                
                ai_chat.add_message(message_content);
                println!("{0}{1}Lector:{2}{3}\n{4}", CONSOLE_GREEN_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR, response.choices[0].message.content);
            }
            Err(error) => {
                eprintln!("Error sending message: {:?}", error);
            }
        }
        println!("\n");
        //println!("{:#?}", ai_chat);
    }
}

async fn send_request(request: &GPTRequest, ywt_api_key: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post(OPENAI_MODEL_URL)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", ywt_api_key.trim()))
        .json(request)
        .send()
        .await
}

fn create_response_parse_error(error: &serde_json::Error) -> GPTResponse {
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

fn create_response_reqwest_error(error: &String) -> GPTResponse {
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

async fn parse_response(response: reqwest::Response) -> Result<GPTResponse, reqwest::Error> {
    match response.status() {
        StatusCode::OK => {
            let response_body = response.text().await?;
            let parse_result: Result<GPTResponse, _> = serde_json::from_str(&response_body);
            match parse_result {
                Ok(parsed_data) => {
                    Ok(parsed_data)
                },
                Err(error) => {
                    Ok(create_response_parse_error(&error))
                },
            }
        }
        _ => {
            Ok(create_response_reqwest_error(&response.text().await?))
        }
    }
}

async fn send_message(request: &GPTRequest, ywt_api_key: &str) -> Result<GPTResponse, reqwest::Error> {
    let response = send_request(request, ywt_api_key).await?;
    let parsed_data = parse_response(response).await?;
    Ok(parsed_data)
}