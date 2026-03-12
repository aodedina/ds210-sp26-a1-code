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

        let asynchronous_output = chat_session.add_message(message); //send users message to chat session
        let output = asynchronous_output.await; //wait for LLM to finish generating response; await pauses any input that may come in

        //handle success and error cases coming from the LLM's response
        let output_text = match output {
            Ok(text) => text,
            Err(_) => String::from("Sorry, couldn't generate a response."), //fallback response
        };
        return output_text;

        // You need to add your code here
        // You must find a way to add the given message to the chat_session!
        // consider https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.add_message
        // Hint: make sure you transform/extract the response message as a **String**.
    }
}