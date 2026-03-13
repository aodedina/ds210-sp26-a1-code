use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    model: Llama,
    chat_session: Option<Chat<Llama>>
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        return ChatbotV2 {
            model, 
            chat_session: None, 
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        if self.chat_session.is_none() {
            self.chat_session = Some(
                self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate")
            );
        }

        let chat_session = self.chat_session.as_mut().unwrap();

        let response = chat_session.add_message(message).await;

        //handle success and error cases coming from the LLM's response
        match response {
            Ok(output) => output.to_string(),
            Err(_) => String::from("Something went wrong."), //fallback response
        }
    }
}
