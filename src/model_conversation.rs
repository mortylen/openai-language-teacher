use reqwest::{Error};

use crate::openai_executor::GPTRequest;
use crate::openai_executor::Message;
use crate::openai_executor::send_message;
use crate::openai_executor::get_user_input;

pub async fn model_conversation(ywt_api_key: &String, mut ai_chat: GPTRequest, target_language: &String, native_language: &String) -> Result<(), Error> {
    let system_message: String = format!("You are my {0} teacher. I would like you to split each of your replies into three part. \nIn the first part called as 'Correction:', write the correct version of my sentence in {0} language. \nIn second part called as 'Note', describle and explain of where I made grammatical mistakes in my {0} language, this part you must described in {1} language. \nIn the third part called as 'Conversation:', feel free to respond to my statement and continue the conversation in {0} language.", &target_language, &native_language);
    println!("\n");
    println!("Please start the conversation.");

    let mut user_message: String;
    loop {
        ai_chat.remove_system_message();
        ai_chat.add_message(Message{role: "system".to_string(), content: system_message.clone()});
        println!("You: "); 
        user_message = format!("Fix my {0}. Correct my mistakes in this paragraph to standard {0} and write the explanation in {1}. \n'{2}'", &target_language, &native_language, get_user_input());
        ai_chat.add_message(Message{role: "user".to_string(), content: user_message});
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
