use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");
            let response = chat_session.add_message(message).await; //sends msg to chatbot and then await tells rust to wait until it's done generating a response
            match response { //checks if response successful
                Ok(output) => output.to_string(), //if function succeeds, converts chatbots response into a string so it can be returned 
                Err(_) => String::from("Something went wrong."), //if function fails, returns the statement in parentheses using String::from (like pure string)
            }
        }
    }
