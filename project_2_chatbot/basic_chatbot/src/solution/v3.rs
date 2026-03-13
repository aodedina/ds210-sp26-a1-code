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
            // Make sure you initialize your struct members here
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let chat_session = if self.chat_sessions.contains_key(&username) {
            self.chat_sessions.get_mut(&username).unwrap()
        } else {
            let new_session = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            
            self.chat_sessions.insert(username.clone(), new_session);
            self.chat_sessions.get_mut(&username).unwrap()
        };
        let response = chat_session.add_message(message).await;
        
        //handle success and error cases coming from the LLM's response
        match response {
            Ok(output) => output.to_string(),
            Err(_) => String::from("Something went wrong."), //fallback response
        }
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        let mut history_strings = Vec::new();
        if let Some(chat) = self.chat_sessions.get(&username) {
            if let Ok(session) = chat.session() {
                let history = session.history();
                println!("{:?}", history);

                
                for i in 0..history.len() {
                    let message = &history[i];
                    let content = message.content().to_string();
                    history_strings.push(content);
                }
            }
            return history_strings;

        } else {
            return Vec::new();
        }
    }
}
