//use std::env;
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

fn main() {
    let mut ai_chat = GPTRequest::new(OPENAI_MODEL.to_string(), OPENAI_TEMPERATURE, OPENAI_MAXTOKENS);
    ai_chat.add_message(Message{role: "system".to_string(), content: "english teacher1".to_string()});
    ai_chat.add_message(Message{role: "system".to_string(), content: "english teacher2".to_string()});
    ai_chat.add_message(Message{role: "system".to_string(), content: "english teacher3".to_string()});
    ai_chat.add_message(Message{role: "system".to_string(), content: "english teacher4".to_string()});
    ai_chat.add_message(Message{role: "system".to_string(), content: "english teacher5".to_string()});
    ai_chat.add_message(Message{role: "system".to_string(), content: "english teacher6".to_string()});

    println!("{:#?}", ai_chat);
}