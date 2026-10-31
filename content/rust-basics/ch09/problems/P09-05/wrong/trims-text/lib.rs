use std::fs;
use std::io::{Error, ErrorKind};

pub fn recovery(error: &Error) -> String {
    let plan = match error.kind() {
        ErrorKind::NotFound => "create",
        ErrorKind::PermissionDenied => "denied",
        ErrorKind::Interrupted => "retry",
        _ => "stop",
    };
    plan.to_string()
}

pub fn load(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text.trim().to_string()),
        Err(error) => Err(recovery(&error)),
    }
}
