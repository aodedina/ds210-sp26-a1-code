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
<<<<<<< HEAD
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
=======
        let chat_session = self.chat_sessions //creates variable that will hold a mutable chat session for this user
        .entry(username)
        .or_insert(
            self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate")
        );

        let response = chat_session.add_message(message).await;  //send message to chat session and wait for chat bot's response
>>>>>>> s2basic
        
        //handle success and failure responses
        match response {
            Ok(output) => output.to_string(),
            Err(_) => String::from("Something went wrong."), //error response
        }
    }


    
    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
<<<<<<< HEAD
        if let Some(chat) = self.chat_sessions.get(&username) { //attempts to find user's session
            if let Some(session) = chat.session() { //retrieves session object
                let history = session.history(); //gets message history from chat sessions
                let mut history_for_strings = Vec::new(); //creates an empty vector to store strings
                for message in history {
                    let pre_message = message.content().to_string(); //converts text to string
                    history_for_strings.push(pre_message); //adds this onto our previously made vector 
=======
        let mut history_strings = Vec::new();  //create empty vector that will store conversation history as strings
        if let Some(chat) = self.chat_sessions.get(&username) {  //check if chat session exists; if so, retrieves it
            if let Ok(session) = chat.session() {
                let history = session.history();  //get message history stored in this session
                println!("{:?}", history);

                
                //loop through each message in history
                for i in 0..history.len() {
                    let message = &history[i];  //access current message in history 
                    let content = message.content().to_string();  //extract text content of message and convert to string 
                    history_strings.push(content);  //add message content to vector of history strings
>>>>>>> s2basic
                }
                return history_for_strings; //returns list
            }
<<<<<<< HEAD
=======
            return history_strings;  

        } else {  //if no chat session exists for username, return empty vector 
            return Vec::new();
>>>>>>> s2basic
        }
        Vec::new(); //if all else return empty list
      }
    }

