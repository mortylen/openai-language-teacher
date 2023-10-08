use std::env;
//use std::io;
use reqwest::{Error};

mod openai_executor;
mod model_conversation;
mod model_words;
//mod model_exercises;

//const OPENAI_MODEL: &'static str = "gpt-4";
const OPENAI_MODEL: &'static str = "gpt-3.5-turbo";
const OPENAI_TEMPERATURE: f32 = 0.8;
const OPENAI_MAXTOKENS: i32 = 1024; //2048; //4096;

fn wait_for_api_ywt() -> String {
    let args: Vec<String> = env::args().collect();
    let api_ywt: String = if args.len() > 1 {
        args[1].to_string()
    } else {
        println!("\n");
        println!("Please enter your OpenAI API key (do not share your API key with others, or expose it in the browser or other client-side code): ");
        openai_executor::get_user_input()
    };

    api_ywt
}

fn set_target_language() -> String {
    println!("\n");
    println!("Please type the language you want to learn (for example 'English'): ");
    openai_executor::get_user_input()
}

fn set_native_language() -> String {
    println!("\n");
    println!("Please type your native language. (for example 'Germany'): ");
    openai_executor::get_user_input()
}

fn set_openai_chat() -> openai_executor::GPTRequest {
    let ai_chat = openai_executor::GPTRequest::new(OPENAI_MODEL.to_string(), OPENAI_TEMPERATURE, OPENAI_MAXTOKENS);
    ai_chat
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ywt_api_key = wait_for_api_ywt();
    let target_language = set_target_language();
    let native_language = set_native_language();
    let ai_chat = set_openai_chat();

    println!("\n");
    println!("Choose your education model: \n1 - Conversation \n2 - Learning words \n\nEnter the number of model:");
    match openai_executor::get_user_input().trim() {
        "1" => model_conversation::model_conversation(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
        "2" => model_words::model_words(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
        //"3" => model_exercises::model_exercises(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
        _ =>model_conversation::model_conversation(&ywt_api_key, ai_chat, &target_language, &native_language).await?,
    }

    Ok(())
}