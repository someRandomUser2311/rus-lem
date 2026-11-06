use std::thread;
use std::time::Duration;

pub fn route(request_line: &str, slow: Duration) -> (u16, String) {
    let words: Vec<&str> = request_line.split(' ').collect();
    match words.as_slice() {
        ["GET", "/", "HTTP/1.1"] => (200, String::from("Hello!")),
        ["GET", "/sleep", "HTTP/1.1"] => {
            thread::sleep(slow);
            (200, String::from("Slept"))
        }
        ["GET", path, "HTTP/1.1"] if path.starts_with("/echo/") => (200, path[6..].to_string()),
        ["GET", _, "HTTP/1.1"] => (404, String::from("Not found")),
        _ => (400, String::from("Bad request")),
    }
}

pub fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Unknown",
    }
}
