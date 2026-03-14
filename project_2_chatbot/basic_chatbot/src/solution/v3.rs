use std::collections::HashMap;  //can store one chat session per user
use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    chat_sessions: HashMap<String, Chat<Llama>>
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model,
            chat_sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        //matches chat session with existing or creates new session
        let chat_session = match self.chat_sessions.get_mut(&username) {
            Some(session) => session,
            None => {
                let new_chat = self.model 
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate.");
                self.chat_sessions.insert(username.clone(), new_chat);
                self.chat_sessions.get_mut(&username).unwrap()   
            }
        };
        
        let response = chat_session.add_message(message).await; //add users new message to the correct chat session
        
        //handles success and failure output responses
        match response {
            Ok(output) => output.to_string(),
            Err(_) => String::from("Something went wrong."), //error response
        }
    }


    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if let Some(chat) = self.chat_sessions.get(&username) { //attempts to find user's session
            //let session =  LlamaChatSession::from_bytes(std::fs::read(format!("sessions/{}.bin", username)).unwrap().as_slice()).unwrap();
            let history = chat.session().unwrap().history(); //gets message history from chat sessions
            let mut history_strings = Vec::new(); //creates an empty vector to store strings
            history_strings.push(String::from("How will this assistant act?"));

            for message in history {
                history_strings.push(message.content().to_string()); //adds this onto our previously made vector 
            }
            return history_strings; //returns list
        } else {
            return Vec::new();
        }
    }
}

