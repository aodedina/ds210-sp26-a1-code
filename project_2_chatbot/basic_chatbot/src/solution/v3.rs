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
        let chat_session; //checks for already active session w username
        if self.chat_sessions.contains_key(&username) { //if user exists gets session that alr exists
            chat_session = self.chat_sessions.get_mut(&username).unwrap();
        } else { //if else, creates new chat session
            self.chat_sessions.insert(
             username.clone(), //clone because we need username later 
             self.model
                   .chat()
                   .with_system_prompt("The assistant will act like a pirate."), 
                   //this will create new chat with above prompt
            );
            chat_session = self.chat_sessions.get_mut(&username).unwrap(); //retrieves new session
        }
        let response = chat_session.add_message(message).await; //add users new message to correct chat session
        
        //handle success and failure responses
        match response {
            Ok(output) => output.to_string(),
            Err(_) => String::from("Something went wrong."), //error response
        }
    }


    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if let Some(chat) = self.chat_sessions.get(&username) { //attempts to find user's session
            if let Ok(session) = chat.session() { //retrieves session object
                let history = session.history(); //gets message history from chat sessions
                let mut history_strings = Vec::new(); //creates an empty vector to store strings

                for message in history {
                    history_strings.push(message.content().to_string()); //adds this onto our previously made vector 
                }
                return history_strings; //returns list
            }
        } 
        Vec::new() //if all else return empty list
    }

}

