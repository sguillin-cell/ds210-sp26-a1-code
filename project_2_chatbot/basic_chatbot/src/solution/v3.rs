use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        ChatbotV3 {
            model,
            sessions: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {

        if !self.sessions.contains_key(&username) {
            let chat = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");

            self.sessions.insert(username.clone(), chat);
        }
        let session = match self.sessions.get_mut(&username) {
            Some(session) => session,
            None => panic!("Session not found"),
        };
        let response = session.add_message(message).await;
        match response {
            Ok(reply) => reply,
            Err(_) => "Failed to respond, sorry!".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        return Vec::new();
    }
}