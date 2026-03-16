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
        
        match self.sessions.get(&username) {
            Some(chat) => {
                
                match chat.session() {
                    Ok(session) => {
                        let history = session.history();
                        let mut result: Vec<String> = Vec::new();
                        for msg in history.iter() {
                            result.push(msg.content().to_string());
                        }
                        return result;
                    }  
                    Err(_) => Vec::new()
                }

            }
            None => {
                return Vec::new()
            }
        }
    }
}