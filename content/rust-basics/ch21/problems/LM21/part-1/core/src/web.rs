use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::library::Library;

pub fn respond(lib: &Library, request_line: &str) -> (u16, String) {
    let words: Vec<&str> = request_line.split(' ').collect();
    let ["GET", path, "HTTP/1.1"] = words.as_slice() else {
        return (400, String::from("Bad request"));
    };
    let parts: Vec<&str> = path.split('/').collect();
    match parts.as_slice() {
        ["", "health"] => (200, String::from("ok")),
        ["", "books"] => (
            200,
            lib.find("").iter().map(|b| format!("{}\n", b.summary())).collect(),
        ),
        ["", "authors"] => (200, lib.authors().iter().map(|a| format!("{a}\n")).collect()),
        ["", "books", id] if !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()) => {
            match id.parse::<u32>().ok().and_then(|n| lib.book(n)) {
                Some(book) => (200, format!("{}\n", book.summary())),
                None => (404, String::from("No such book")),
            }
        }
        _ => (404, String::from("Not found")),
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Bad Request",
    }
}

fn handle(lib: &Mutex<Library>, mut stream: TcpStream) -> io::Result<()> {
    let mut line = String::new();
    let n = {
        let mut reader = BufReader::new(&mut stream);
        let n = (&mut reader).take(8192).read_line(&mut line);
        let _ = io::copy(&mut (&mut reader).take(1 << 20), &mut io::sink());
        n
    };
    let (status, body) = match n {
        Ok(_) => {
            let guard = lib.lock().unwrap_or_else(|e| e.into_inner());
            respond(&guard, line.trim_end_matches(['\r', '\n']))
        }
        Err(_) => (400, String::from("Bad request")),
    };
    let reply = format!(
        "HTTP/1.1 {status} {}\r\nContent-Length: {}\r\n\r\n{body}",
        reason(status),
        body.len()
    );
    stream.write_all(reply.as_bytes())
}

pub fn serve(lib: Library, listener: TcpListener, workers: usize, max_requests: usize) -> io::Result<usize> {
    let lib = Arc::new(Mutex::new(lib));
    let mut running: VecDeque<thread::JoinHandle<()>> = VecDeque::new();
    let mut handled = 0;
    for stream in listener.incoming().take(max_requests) {
        let stream = stream?;
        if running.len() >= workers.max(1) {
            if let Some(oldest) = running.pop_front() {
                let _ = oldest.join();
            }
        }
        let lib = Arc::clone(&lib);
        running.push_back(thread::spawn(move || {
            let _ = handle(&lib, stream);
        }));
        handled += 1;
    }
    for h in running {
        let _ = h.join();
    }
    Ok(handled)
}

/// A test client (given): sends `raw`, closes its write side, and reads the whole reply.
pub fn fetch(port: u16, raw: &str) -> io::Result<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    stream.write_all(raw.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;
    let mut reply = String::new();
    stream.read_to_string(&mut reply)?;
    Ok(reply)
}
