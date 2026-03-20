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
    let bytes = session.to_bytes().unwrap(); //converts session to bytes so it can be stored kalosm
    //fs writes into a file to save
    match fs::write(filename, bytes) {
        Ok(_) => {} //empty do nothing
        Err(_) => panic!("Couldn't write to session file"), //panics if theres an error
    }
    
}

// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    let bytes = fs::read(filename).ok()?;
    
    LlamaChatSession::from_bytes(&bytes).ok()
}