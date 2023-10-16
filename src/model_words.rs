use reqwest::{Error};

use crate::openai_executor::GPTRequest;
use crate::openai_executor::Message;
use crate::openai_executor::send_message;
use crate::openai_executor::get_user_input;

pub async fn model_words(ywt_api_key: &String, mut ai_chat: GPTRequest, target_language: &String, native_language: &String) -> Result<(), Error> {
    let system_message: String = format!("You are my {0} language lector. You will generate 10 random words in {1} language and I will translate them into {0} language. You will check and correct my translation.", &target_language, &native_language);
	let mut first_loop = true;
    let mut user_message: String;
    let mut openai_words: String = String::from("");
    println!("\n");
    
    loop {
        ai_chat.remove_system_message();   
        ai_chat.add_message(Message{role: "system".to_string(), content: system_message.clone()});
		if first_loop {
			first_loop = false;
            user_message = format!("Please generate 10 random words only in {0} language, you don't translate them. And write nothing more.", &native_language);
		}
		else {
			println!("Translate the words: ");
            user_message = format!("Please correct my translate from {1} language to {0} language. Write where I made a mistake, write it in {1} language. Use those words in simple sentences in {0} language.\n This is {1} words:{3} \nThis is my translation:\n{2}", &target_language, &native_language, get_user_input(), &openai_words);
            first_loop = true;
            println!("\n");
		}
        ai_chat.add_message(Message{role: "user".to_string(), content: user_message});
        
        match send_message(&ai_chat, &ywt_api_key).await {
            Ok(response) => {
                let message_content = Message {
                    role: response.choices[0].message.role.clone(),
                    content: response.choices[0].message.content.clone(),
                };
                
                ai_chat.add_message(message_content);
                println!("Lector: \n{0}", response.choices[0].message.content);
                //println!("{:#?}", response.usage);
                openai_words = response.choices[0].message.content.clone();
            }
            Err(error) => {
                eprintln!("Error sending message: {:?}", error);
            }
        }
        println!("\n");
    }
}