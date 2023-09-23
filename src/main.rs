use std::env;
use std::io;
use std::collections::HashMap;
//use std::error::Error;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value, json};
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
//use reqwest::Client;
use reqwest::{Client, Error, header};

const MAX_MESSAGE_BUFFER: usize = 10;
//const OPENAI_MODEL: &'static str = "gpt-4";
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
    //let message = format!("You are my lector of {0} language. You will be provided with statements, and your task will be to split the answer in two. First: convert my statements to standard {0} language. Second: keep the conversation going.", language);
    let message = format!("You are my lector of {0} language.", language);
    //let message = format!("Could you please assist me with something? I have a specific request for our conversation format. I would like you to split each of your replies into two paragraphs. In the first paragraph, I kindly ask you to correct any mistakes in my {0} language. In the second paragraph, feel free to respond to my statement and continue the conversation.", language);
    //let message = format!("You will be provided with statements, and your task will be to split the answer in two. First: convert my statements to standard {0} language. Second: keep the conversation going.", language);
    ai_chat.add_message(Message{role: "system".to_string(), content: message});
    //ai_chat.add_message(Message{role: "user".to_string(), content: message});
    ai_chat
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ywt_api_key = wait_for_api_ywt();
    let language = set_language();
    println!("ywt: {}", ywt_api_key);
    println!("Language: {}", language);

    let mut ai_chat = set_openai_chat(&language);

    //send_message(&ai_chat, &ywt_api_key).await?;

    println!("Please start the conversation: ");
    
    //let mut ai_chat = GPTRequest::new(OPENAI_MODEL.to_string(), OPENAI_TEMPERATURE, OPENAI_MAXTOKENS);
    //ai_chat.add_message(Message{role: "user".to_string(), content: "english teacher1".to_string()});
    //ai_chat.add_message(Message{role: "assistant".to_string(), content: "english teacher2".to_string()});
    //ai_chat.add_message(Message{role: "user".to_string(), content: "english teacher3".to_string()});
    //ai_chat.add_message(Message{role: "assistant".to_string(), content: "english teacher4".to_string()});
    //ai_chat.add_message(Message{role: "user".to_string(), content: "english teacher5".to_string()});
    //ai_chat.add_message(Message{role: "assistant".to_string(), content: "english teacher6".to_string()});

    //println!("{:#?}", ai_chat);
    for n in 1..11 {
    //loop {
    println!("You: ");
    //let message: String = get_user_input();
    let message: String = format!("I would like you to split each of your replies into two paragraphs. In the first paragraph, I kindly ask you to correct any mistakes in my {0} language. In the second paragraph, feel free to respond to my statement and continue the conversation. \n\n '{1}'", language,  get_user_input());
    ai_chat.add_message(Message{role: "user".to_string(), content: message});
    //let response_content: String = get_content_from_response(send_message(message));
    //ai_chat.add_message(Message{role: "assistant".to_string(), content: response_content});
    //println!("Lector: {}", response_content);
    //break;
    //}

    //println!("{:#?}", ai_chat);

    
    //send_message(&ai_chat, &ywt_api_key).await?;
    match send_message(&ai_chat, &ywt_api_key).await {
        Ok(response) => {
            let message_content = Message {
                role: response.choices[0].message.role.clone(),
                content: response.choices[0].message.content.clone(),
            };
            
            ai_chat.add_message(message_content);
        }
        Err(error) => {
            // Handle the error case here
            eprintln!("Error sending message: {:?}", error);
        }
    }

    println!("{:#?}", ai_chat);
    }
    
    Ok(())

    //if let Err(err) = send_message(&ai_chat, &ywt_api_key).await {
    //    eprintln!("Error: {}", err);
    //}
}

async fn send_message(request: &GPTRequest, ywt_api_key: &String) -> Result<GPTResponse, Error>  {
    let url = "https://api.openai.com/v1/chat/completions"; // "https://httpbin.org/post";
    //let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", ywt_api_key.trim()))
        //.body(json_data.to_owned())
        .json(&request)
        .send()
        .await?;

    //TODO: Check status
    println!("Status: {}", response.status());

    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    //let xxx: GPTResponse = serde_json::from_str(&response_body);

    let parse_result: Result<GPTResponse, _> = serde_json::from_str(&response_body);
    match parse_result {
        Ok(parsed_data) => {
            println!("Parsed data: {:#?}", parsed_data);
            println!("MESSAGE: {:#?}", parsed_data.choices[0].message);
            Ok(parsed_data)
        },
        Err(error) => {
            println!("Error parsing JSON: {:#?}", error);
            let empty_response: GPTResponse = Default::default();
            Ok(empty_response)
         },
     }

    //Ok(())
}

// async fn send_message(request: &GPTRequest, ywt_api_key: &String) -> Result<(), Error>  {
//     let url = "https://api.openai.com/v1/chat/completions"; // "https://httpbin.org/post";
//     //let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

//     let client = reqwest::Client::new();

//     let response = client
//         .post(url)
//         .header(CONTENT_TYPE, "application/json")
//         .header(AUTHORIZATION, format!("Bearer {}", ywt_api_key.trim()))
//         //.body(json_data.to_owned())
//         .json(&request)
//         .send()
//         .await?;

//     println!("Status: {}", response.status());

//     let response_body = response.text().await?;
//     println!("Response body:\n{}", response_body);

//     //let xxx: GPTResponse = serde_json::from_str(&response_body);

//     let parse_result: Result<GPTResponse, _> = serde_json::from_str(&response_body);
//     match parse_result {
//         Ok(parsed_data) => {
//              println!("Parsed data: {:#?}", parsed_data);
//             println!("MESSAGE: {:#?}", parsed_data.choices[0].message);
//         },
//         Err(error) => {
//              println!("Error parsing JSON: {:#?}", error);
//          },
//      }

//     Ok(())
// }

