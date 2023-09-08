use std::env;
use std::io;
//use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
//use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};

const MAX_MESSAGE_BUFFER: usize = 10;
const OPENAI_MODEL: &'static str = "gpt-3.5-turbo";
const OPENAI_TEMPERATURE: f32 = 0.8;
const OPENAI_MAXTOKENS: i32 = 2048; 

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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GPTError {
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
        //let mut user_input = String::new();
        //io::stdin().read_line(&mut user_input).unwrap();
        //user_input.trim().to_string()
    };

    api_ywt
}

fn set_language() -> String {
    println!("Please type the language you want to learn (for example 'English'): ");
    get_user_input()
    //let mut user_input = String::new();
    //io::stdin().read_line(&mut user_input).unwrap();
    //user_input.trim().to_string()
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn set_openai_chat(language: &String) -> GPTRequest {
    let mut ai_chat = GPTRequest::new(OPENAI_MODEL.to_string(), OPENAI_TEMPERATURE, OPENAI_MAXTOKENS);
    let message = format!("You are my lector of {0} language. You will be provided with statements, and your task will be to split the answer in two. First: convert my statements to standard {0} language. Second: keep the conversation going.", language);
    ai_chat.add_message(Message{role: "system".to_string(), content: message});
    ai_chat
}

fn main() {
    let ywt_api_key = wait_for_api_ywt();
    let language = set_language();
    println!("ywt: {}", ywt_api_key);
    println!("Language: {}", language);

    let mut ai_chat = set_openai_chat(&language);

    println!("Please start the conversation: ");
    
    //let mut ai_chat = GPTRequest::new(OPENAI_MODEL.to_string(), OPENAI_TEMPERATURE, OPENAI_MAXTOKENS);
    ai_chat.add_message(Message{role: "user".to_string(), content: "english teacher1".to_string()});
    ai_chat.add_message(Message{role: "assistant".to_string(), content: "english teacher2".to_string()});
    ai_chat.add_message(Message{role: "user".to_string(), content: "english teacher3".to_string()});
    ai_chat.add_message(Message{role: "assistant".to_string(), content: "english teacher4".to_string()});
    ai_chat.add_message(Message{role: "user".to_string(), content: "english teacher5".to_string()});
    ai_chat.add_message(Message{role: "assistant".to_string(), content: "english teacher6".to_string()});

    println!("{:#?}", ai_chat);
}