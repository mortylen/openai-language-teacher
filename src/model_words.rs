use reqwest::{Error};

use crate::openai_executor::GPTRequest;
use crate::openai_executor::Message;
use crate::openai_executor::send_message;
use crate::openai_executor::get_user_input;

pub async fn model_words(ywt_api_key: &String, mut ai_chat: GPTRequest, target_language: &String, native_language: &String) -> Result<(), Error> {
    let system_message1: String = format!("Generate 10 random words only in {1} language, you don't translate them. I will try to translate them into {0} language in my next answer.", &target_language, &native_language);
    let system_message2: String = format!("You will my {0} language lector. I would like you to split each of your replies into two part. \nIn the first part called as 'Correction:', you will correct and explain my mistakes in my translate words from the previous sentence (example: {1} word - {0} word). And use those words in simple sentences. \nIn second pard called as 'Translate the words:', generate 10 random words in {1} language and I will try to translate them into {0} language in my next answer.", &target_language, &native_language);
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
			println!("Translate the words: ");
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
                println!("Lector: \n{0}", response.choices[0].message.content);
                //println!("{:#?}", response.usage);
            }
            Err(error) => {
                eprintln!("Error sending message: {:?}", error);
            }
        }
        println!("\n");
    }
}
