use std::collections::HashMap;  //can store one chat session per user
use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    chat_sessions: HashMap<String, Chat<Llama>>

    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
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
        let chat_session = self.chat_sessions
            .entry(username)
            .or_insert(
                self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate")
            );

        let asynchronous_output = chat_session.add_message(message); //send users message to chat session
        let output = asynchronous_output.await; //wait for LLM to finish generating response; await pauses any input that may come in

        //handle success and error cases coming from the LLM's response
        let output_text = match output {
            Ok(text) => text,
            Err(_) => String::from("Sorry, couldn't generate a response."), //fallback response
        };
        return output_text;

        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if let Some(chat) = self.chat_sessions.get(&username) {
            let session = chat.session().unwrap();
            let history = session.history();

            println!("{:?}", history);

            let mut history_strings = Vec::new();

            
            for i in 0..history.len() {
                let message = &history[i];
                let content = message.content().to_string();
                history_strings.push(content);
            }

            return history_strings;

        }
            return Vec::new();

        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        
    }
}