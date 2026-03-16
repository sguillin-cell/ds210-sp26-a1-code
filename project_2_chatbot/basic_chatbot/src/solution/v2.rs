use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    model: Llama,
    session: Chat<Llama>
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        
        let session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        return ChatbotV2 {
            model,
            session,
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {

        let response = self.session.add_message(message).await;

        match response {
            Ok(reply) => reply,
            Err(_) => "Failed to respond, sorry!".to_string(), 
        }
    }
}