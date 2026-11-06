use std::io::{self, Write};

pub fn response(status: u16, reason: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\n\r\n{body}",
        body.chars().count()
    )
}

pub fn write_response(writer: &mut impl Write, status: u16, reason: &str, body: &str) -> io::Result<()> {
    writer.write_all(response(status, reason, body).as_bytes())?;
    writer.flush()
}
