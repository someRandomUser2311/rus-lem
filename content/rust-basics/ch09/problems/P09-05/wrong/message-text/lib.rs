use std::fs;
use std::io::Error;

pub fn recovery(error: &Error) -> String {
    let m = error.to_string();
    if m.contains("not found") {
        return String::from("create");
    }
    String::from("stop")
}

pub fn load(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(error) => Err(recovery(&error)),
    }
}
