use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username); //attempt to retrieve user's chat from cache

        //check if chat found in cache
        match cached_chat {
            //conversation found in cache
            Some(chat) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                let response = chat(&message).await.unwrap(); //send user's message to chatbot; wait for response

                if let Ok(session) = chat.session() {
                    file_library::save_chat_session_to_file(filename, &session); //save updated chat session to file
                }
                  
                return response  //return chatbot response
            }

            //conversation not found in cache
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                
                let mut chat = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");  //create new chat session with assistant as pirate message prompt

                //load previously saved session from file
                if let Some(session) = file_library::load_chat_session_from_file(filename) {
                    chat = chat.with_session(session);  //restore the past conversation
                }
                let response = chat(&message).await.unwrap();  

                if let Ok(session) = chat.session() {
                    file_library::save_chat_session_to_file(&filename, &session);
                }
            
                self.cache.insert_chat(username.clone(), chat);  //insert this chat into cache (now most recently used chat)

                return response;
            }
        }
    }





    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => { //if not in cache will load from file
                //load chat from session file
                match file_library::load_chat_session_from_file(filename) {
                    Some(session) => { //if file exists loads session
                        let chat = self.model //creates new chat and attaches previous session
                            .chat()
                            .with_system_prompt("The assistant will act like a pirate")
                            .with_session(session);
                        self.cache.insert_chat(username.clone(), chat.clone()); //adds onto cache
                        //takes out the session from the chat
                        if let Ok(session) = chat.session() { 
                            let history = session.history(); //gets past messages
                            let mut history_strings = Vec::new(); //new vector to store strings
                            for message in history.iter().skip(1) { //loops through each message
                                history_strings.push(message.content().to_string()); //converts messages to strings and stores it
                            }
                            return history_strings;
                        }
                    }
                    None => { //if file doesn't exist then no history
                        return Vec::new();
                    }
                }
                Vec::new() //fallback if something happens
            }
            Some(chat_session) => { //if found in cache
                println!("get_history: {username} is in the cache! Nice!");
                //gets new session from the cached chat
                if let Ok(session) = chat_session.session() {
                    let history = session.history(); //gets history 
                    let mut history_strings = Vec::new();
                    for message in history.iter().skip(1) {
                        history_strings.push(message.content().to_string());
                    }
                    return history_strings;
                }
                Vec::new() //fallback again in case something happens
            }
        }
    }
}
