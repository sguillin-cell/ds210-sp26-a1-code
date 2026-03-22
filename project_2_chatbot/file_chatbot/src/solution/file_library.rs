use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    match session.to_bytes() {
        Ok(bytes) => {
            match std::fs::write(filename, bytes) {
                Ok(_) => println!("Session saved successfully."),
                Err(e) => panic!("Failed to write file: {} ",e),
            }
        }
        Err(e) => {
            eprintln!("Failed to Open session: {}", e);
        }
    }
}

// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    match fs::read(filename) {
        Ok(bytes) => {
            match LlamaChatSession::from_bytes(bytes.as_slice()) {
                Ok(session) => Some(session),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}