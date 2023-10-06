use reqwest::{Error};

use crate::openai_executor::GPTRequest;
use crate::openai_executor::Message;
use crate::openai_executor::send_message;
use crate::openai_executor::get_user_input;

pub async fn model_exercises(ywt_api_key: &String, mut ai_chat: GPTRequest, target_language: &String, native_language: &String) -> Result<(), Error> {
	println!("\n");
    println!("Set your language level (A1, A2, B1, B2, C1, C2): "); 
	let level_language: String = get_user_input();
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
			println!("Elaborate the exercise: "); 
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
                println!("Lector:\n{0}", response.choices[0].message.content);
                //println!("{:#?}", response.usage);
            }
            Err(error) => {
                eprintln!("Error sending message: {:?}", error);
            }
        }
        println!("\n");

        //println!("{:#?}", ai_chat);
    }
}
