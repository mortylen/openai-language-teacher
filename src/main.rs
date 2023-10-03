use std::env;
use std::io;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value, json};
//use reqwest::{Client, Error, header};
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use reqwest::{Error, StatusCode};

const MAX_MESSAGE_BUFFER: usize = 11;
//const OPENAI_MODEL: &'static str = "gpt-4";
const OPENAI_MODEL: &'static str = "gpt-3.5-turbo";
const OPENAI_MODEL_URL: &'static str = "https://api.openai.com/v1/chat/completions";
const OPENAI_TEMPERATURE: f32 = 0.8;
const OPENAI_MAXTOKENS: i32 = 1024; //2048; //4096;
const CONSOLE_RED_COLOR: &'static str = "\u{1b}[31m";
const CONSOLE_GREEN_COLOR: &'static str = "\u{1b}[32m";
const CONSOLE_BLUE_COLOR: &'static str = "\u{1b}[34m";
const CONSOLE_BROWN_COLOR: &'static str = "\u{1b}[93m";
const CONSOLE_PURPLE_COLOR: &'static str = "\u{1b}[35m";
const CONSOLE_RESET_COLOR: &'static str = "\u{1b}[0m"; //"\u{1b}[39m";
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
            //self.messages.remove(1);
            //self.messages.remove(0);
            self.messages.drain(0..2);
        }
        self.messages.push(msg);
    }

    // fn add_system_message(&mut self, target_language: &String, native_language: &String) {
    //     let message: String = format!("I would like you to split each of your replies into three part. In the first part called as '{0}Correction:{1}', write the correct version of my sentence in {6} language. In second pard called as '{2}Note{3}', description of where I made grammatical mistakes in my {6} language, this part you write in {7} language. In the third part called as '{4}Conversation:{5}', feel free to respond to my statement and continue the conversation in {6} language.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, CONSOLE_PURPLE_COLOR, CONSOLE_RESET_COLOR, &target_language, &native_language);
    //     //let message: String = format!("I would like you to split each of your replies into three part. In the first part called as '{0}Correction:{1}', write the correct version of my sentence {6} language. In second pard called as '{2}Note{3}', description where I made mistakes in my {6} language. In the third part called as '{4}Conversation:{5}', feel free to respond to my statement and continue the conversation.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, CONSOLE_PURPLE_COLOR, CONSOLE_RESET_COLOR, &language);
    //     self.messages.push(Message{role: "system".to_string(), content: message});
    // }

    fn add_system_message(&mut self, system_message: &String) {
        //let message: String = format!("I would like you to split each of your replies into three part. In the first part called as '{0}Correction:{1}', write the correct version of my sentence in {6} language. In second pard called as '{2}Note{3}', description of where I made grammatical mistakes in my {6} language, this part you write in {7} language. In the third part called as '{4}Conversation:{5}', feel free to respond to my statement and continue the conversation in {6} language.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, CONSOLE_PURPLE_COLOR, CONSOLE_RESET_COLOR, &target_language, &native_language);
        //let message: String = format!("I would like you to split each of your replies into three part. In the first part called as '{0}Correction:{1}', write the correct version of my sentence {6} language. In second pard called as '{2}Note{3}', description where I made mistakes in my {6} language. In the third part called as '{4}Conversation:{5}', feel free to respond to my statement and continue the conversation.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, CONSOLE_PURPLE_COLOR, CONSOLE_RESET_COLOR, &language);
        self.messages.push(Message{role: "system".to_string(), content: system_message.to_string()});
    }

    fn remove_system_message(&mut self) {
        if self.messages.len() >= 1 {
            //let msg_index = self.messages.iter().position(|i| i.role == "system").unwrap();
            match self.messages.iter().position(|i| i.role == "system") {
                Some(msg_index) => {
                    self.messages.remove(msg_index);
                },
                None => {},
            }
            //self.messages.remove(msg_index);
        }
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

fn set_target_language() -> String {
    println!("Please type the language you want to learn (for example 'English'): ");
    get_user_input()
}

fn set_native_language() -> String {
    println!("Please type your native language. (for example 'Germany'): ");
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
    let target_language = set_target_language();
    let native_language = set_native_language();
    let ai_chat = set_openai_chat(&target_language);

    println!("Choose your education model: \n1 - Conversation \n2 - Learning words \n3 - Exercises \nEnter the number of model:");
    match get_user_input().trim() {
        "1" => model_conversation(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
        "2" => model_words(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
        "3" => model_exercises(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
        _ => model_conversation(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
    }

    Ok(())
}

async fn model_conversation(ywt_api_key: &String, mut ai_chat: GPTRequest, target_language: &String, native_language: &String) -> Result<(), Error> {
	let system_message: String = format!("I would like you to split each of your replies into three part. \nIn the first part called as '{0}Correction:{1}', write the correct version of my sentence in {6} language. \nIn second pard called as '{2}Note{3}', description and explain of where I made grammatical mistakes in my {6} language, this part you write in {7} language. \nIn the third part called as '{4}Conversation:{5}', feel free to respond to my statement and continue the conversation in {6} language.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, CONSOLE_PURPLE_COLOR, CONSOLE_RESET_COLOR, &target_language, &native_language);
    println!("Please start the conversation.");
    
    loop {
        ai_chat.remove_system_message();
		ai_chat.add_system_message(&system_message);
        println!("{0}{1}You: {2}{3}", CONSOLE_BLUE_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR); 
        ai_chat.add_message(Message{role: "user".to_string(), content: get_user_input()});
        println!("\n");
        
        match send_message(&ai_chat, &ywt_api_key).await {
            Ok(response) => {
                let message_content = Message {
                    role: response.choices[0].message.role.clone(),
                    content: response.choices[0].message.content.clone(),
                };
                
                ai_chat.add_message(message_content);
                println!("{0}{1}Lector:{2}{3}\n{4}", CONSOLE_GREEN_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR, response.choices[0].message.content);
                println!("{:#?}", response.usage);
            }
            Err(error) => {
                eprintln!("Error sending message: {:?}", error);
            }
        }
        println!("\n");
    }
}

async fn model_words(ywt_api_key: &String, mut ai_chat: GPTRequest, target_language: &String, native_language: &String) -> Result<(), Error> {
	//let system_message: String = format!("You will my {4} language lector. I would like you to split each of your replies into two part. \nIn the first part called as '{0}Correction:{1}', correctand and explain my mistakes in words based on the previous answer (if exist). Explain my mistakes in {5} language and correct words in {4} language. \nIn second pard called as '{2}Translate the words:{3}', generate 10 {5} words in {5} language (format: {5} word, {5} word, {5} word...) and I will translate them in my answer to {4} language.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, &target_language, &native_language);
    let system_message1: String = format!("Generate 10 random words only in {1} language, you don't translate them. I will try to translate them into {0} language in my next answer.", &target_language, &native_language);
    let system_message2: String = format!("You will my {4} language lector. I would like you to split each of your replies into two part. \nIn the first part called as '{0}Correction:{1}', you will correct and explain my mistakes in my translate words from the previous sentence (example: {5} word - {4} word). And use those words in simple sentences. \nIn second pard called as '{2}Translate the words:{3}', generate 10 random words in {5} language and I will try to translate them into {4} language in my next answer.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, &target_language, &native_language);
	let mut first_loop = true;
    loop {
        ai_chat.remove_system_message();
		//ai_chat.add_system_message(&system_message);
		
		if first_loop {
			first_loop = false;
            ai_chat.add_system_message(&system_message1);
			ai_chat.add_message(Message{role: "user".to_string(), content: "Please start generate the words.".to_string()});
		}
		else {
			println!("{0}{1}Translate the words: {2}{3}", CONSOLE_BLUE_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR);
            ai_chat.add_system_message(&system_message2);
			ai_chat.add_message(Message{role: "user".to_string(), content: get_user_input()});
		}

        println!("\n");
        
        match send_message(&ai_chat, &ywt_api_key).await {
            Ok(response) => {
                let message_content = Message {
                    role: response.choices[0].message.role.clone(),
                    content: response.choices[0].message.content.clone(),
                };
                
                ai_chat.add_message(message_content);
                println!("{0}{1}Lector:{2}{3}\n{4}", CONSOLE_GREEN_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR, response.choices[0].message.content);
                println!("{:#?}", response.usage);
            }
            Err(error) => {
                eprintln!("Error sending message: {:?}", error);
            }
        }
        println!("\n");
    }
}

async fn model_exercises(ywt_api_key: &String, mut ai_chat: GPTRequest, target_language: &String, native_language: &String) -> Result<(), Error> {
	println!("{0}{1}Set your language level (A1, A2, B1, B2, C1, C2): {2}{3}", CONSOLE_BLUE_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR); 
	let level_language: String = get_user_input();
    // let system_message1: String = format!("You will my {0} language lector. Generate one random exercise in difficulty {1} in {0} language. I will try to work it out in my next answer.", &target_language, &level_language);
    // let system_message2: String = format!("You will my {0} language lector. I would like you to split each of your replies into two part. \nIn the first part called as 'Corection:',  evaluate and correct my answer. Write the correct answer in {0} language, but explain my mistakes in {1} language. \nIn second pard called as 'Exercise:', Generate one random exercise in difficulty {2} in {0} language. I will try to work it out in my next answer.", &target_language, &native_language, &level_language);
	//let system_message1: String = format!("You will my {0} language lector. Generate one random exercise in difficulty {2} in {0} language. This exercise must be in format: Exercise Instructions - this must be written in bilingual, {0} and {1} language. The Exercise - shall not be written in bilingual. Do not disclose the answers from the exercise in the assignment. I will try to work it out in my next answer.", &target_language, &native_language, &level_language);
    //let system_message2: String = format!("You will my {0} language lector. Comunicate with me in {1} language. I would like you to split each of your replies into two part. \nIn the first part called as 'Correction:',  evaluate and correct my answer. Write the correct answer in {0} language, but explain my mistakes in {1} language. \nIn second pard called as 'Exercise:', Generate one random exercise in difficulty {2} in {0} language. And translate your exercise instruction to {1} language. I will try to work it out in my next answer.", &target_language, &native_language, &level_language);
	//let system_message: String = format!("You will my {4} language lector. You will generate exercises in difficulty {6}, you will correct and evaluate my answer. I would like you to split each of your replies into two part. \nIn the first part called as '{0}Correction:{1}', evaluate and correct my answer in {5} language. \nIn second pard called as '{2}Exercise:{3}', generate random exercises in difficulty {6} in {4} language.", CONSOLE_RED_COLOR, CONSOLE_RESET_COLOR, CONSOLE_BROWN_COLOR, CONSOLE_RESET_COLOR, &target_language, &native_language, &level_language);
    let system_message1: String = format!("You are my {0} language lector. Comunicate with me in {1} language. Your job is to teach me {0} language. Generate one random exercise for learning {0} language in difficulty {2}. I will try to work it out in my next answer.", &target_language, &native_language, &level_language);
    let system_message2: String = format!("You are my {0} language lector. Comunicate with me in {1} language. Your job is to teach me {0} language. I would like you to split each of your replies into two part. \nIn the first part called as 'Correction:',  evaluate my answers to the previous exercise. \nIn second pard called as 'Exercise:', Generate one random exercise for learning {0} in difficulty {2}. I will try to work it out in my next answer.", &target_language, &native_language, &level_language);
    
    //println!("Please start the conversation.");
    
	let mut first_loop = true;
    loop {
        ai_chat.remove_system_message();
		//ai_chat.add_system_message(&system_message);
		
		if first_loop {
			first_loop = false;
            ai_chat.add_system_message(&system_message1);
            ai_chat.add_message(Message{role: "user".to_string(), content: "".to_string()});
			//ai_chat.add_message(Message{role: "user".to_string(), content: "Please start generate the exercises.".to_string()});
		}
		else {
			println!("{0}{1}Elaborate the exercise: {2}{3}", CONSOLE_BLUE_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR); 
            ai_chat.add_system_message(&system_message2);
			ai_chat.add_message(Message{role: "user".to_string(), content: get_user_input()});
		}

        println!("\n");
        
        match send_message(&ai_chat, &ywt_api_key).await {
            Ok(response) => {
                let message_content = Message {
                    role: response.choices[0].message.role.clone(),
                    content: response.choices[0].message.content.clone(),
                };
                
                ai_chat.add_message(message_content);
                println!("{0}{1}Lector:{2}{3}\n{4}", CONSOLE_GREEN_COLOR, CONSOLE_BOLD_STYLE, CONSOLE_RESET_BOLD, CONSOLE_RESET_COLOR, response.choices[0].message.content);
                println!("{:#?}", response.usage);
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

fn create_response_error<T>(error: T) -> GPTResponse where T: std::fmt::Display, {
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
                    Ok(create_response_error(&error))
                },
            }
        }
        _ => {
            Ok(create_response_error(&response.text().await?))
        }
    }
}

async fn send_message(request: &GPTRequest, ywt_api_key: &str) -> Result<GPTResponse, reqwest::Error> {
    let response = send_request(request, ywt_api_key).await?;
    let parsed_data = parse_response(response).await?;
    Ok(parsed_data)
}