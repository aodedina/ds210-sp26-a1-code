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
                let session = chat.session().unwrap();  //get current session from the chat
                file_library::save_chat_session_to_file(&filename, &session);  //save updated chat session to file

                response  //return chatbot response
            }

            //conversation not found in cache
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                
                let mut chat = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");  //create new chat session with assistant as pirate message prompt

                //load previously saved session from file
                if let Some(session) = file_library::load_chat_session_from_file(&filename) {
                    chat = chat.with_session(session);  //restore the past conversation
                }

                let response = chat(&message).await.unwrap();  
                let session = chat.session().unwrap();
                file_library::save_chat_session_to_file(&filename, &session);

                self.cache.insert_chat(username.clone(), chat.clone());  //insert this chat into cache (now most recently used chat)

                response
            }
        }
    }





    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                return Vec::new();
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                return Vec::new();

            }
        }
    }
}