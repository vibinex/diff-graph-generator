use once_cell::sync::Lazy;
use std::sync::Mutex;

// Define a global variable
static PROMPT_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn set_prompt_path(path: String) {
    let mut prompt_path = PROMPT_PATH.lock().unwrap();
    *prompt_path = path;
}

pub fn get_prompt_path() -> String {
    let prompt_path = PROMPT_PATH.lock().unwrap();
    prompt_path.clone()
}