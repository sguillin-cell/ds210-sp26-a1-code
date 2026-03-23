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
        let cached_chat = self.cache.get_chat(&username);

        let mut chat = match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                match file_library::load_chat_session_from_file(&filename) {
                    Some(session) => {
                        self.model
                        .chat()
                        .with_system_prompt("The assistent will act like a pirate")
                        .with_session(session)
                    }
                    None => {
                        self.model
                        .chat()
                        .with_system_prompt("The assistent will act like a pirate")
                    }
                }
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                chat_session.clone()
            }
        };

        let response = chat.add_message(message).await;
        {
            let session = chat.session().unwrap();
            file_library::save_chat_session_to_file(&filename, &session);
        }
        self.cache.insert_chat(username.clone(), chat);
        match response {
            Ok(reply) => reply.to_string(),
            Err(_) => "Error".to_string(),
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                match file_library::load_chat_session_from_file(filename) {
                    None => return Vec::new(),
                    Some(session) => messages_from_session(&session),
                }
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                match chat_session.session() {
                    Ok(session) => {
                        let history = session.history();
                        let mut result = Vec::new();
                        for msg in history.iter().skip(1) {
                            result.push(msg.content().to_string());
                        }
                        result
                    }
                    Err(_) => Vec::new(),
                }
            }
        }
    }
}