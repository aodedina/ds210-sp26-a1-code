use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        //loads exisiting session from the file
        match file_library::load_chat_session_from_file(filename) {
            Some(previous_sess) => {
                chat_session = chat_session.with_session(previous_sess);//loads prev session and attaches to chat_session
            }
            None => {} //if not exisiting, do nothing, so like new session
        }
        //sends message from the user to chatbot
        let response = chat_session.add_message(message).await; 
        
        if let Ok(session) = chat_session.session() { //gets session object then saves updated session to the file
            file_library::save_chat_session_to_file(filename, &session);
        }
        match response{ //takes care of cases for response
            Ok(output) => output.to_string(),
            Err(_) => String::from("Something went wrong."),
        }
    }



    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new(); //return empty history if no file exists
            }, 
            Some(session) => {  //session exists
                let mut history_strings = Vec::new();  //makes new vector to store message history as strings

                let history = session.history();  //get full message history from session
                for message in history.iter().skip(1) {  //skips over the assistant will act like a pirate line 
                    history_strings.push(message.content().to_string());  //convert messages to strings and pushes them to vector
                }
                history_strings  //returns full list of past messages
            }
        }
    }
}