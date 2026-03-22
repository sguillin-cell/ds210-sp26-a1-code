use kalosm::language::*;
use crate::solution::file_library::{self, load_chat_session_from_file,save_chat_session_to_file};

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");
        let file = load_chat_session_from_file(&filename);

    match file {
        Some(session) => {
            chat_session = chat_session.with_session(session);
        }
            None => {
        }
    } 
        let response = chat_session.add_message(message).await;
        match response {
            Ok(reply) => {
                if let Ok(session) = chat_session.session() {
                    save_chat_session_to_file(filename, &session);
                }
                reply
            }
            Err(_) => "Failed to respond, sorry!".to_string(),
        }
    }
    
    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                let history = session.history();
                let mut result = Vec::new();
                for msg in history.iter().skip(1) {
                    result.push(msg.content().to_string());
                }
                return result;
            }
        }
    }
}