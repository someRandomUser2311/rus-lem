use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::library::Library;

pub fn respond(lib: &Library, request_line: &str) -> (u16, String) {
    (400, String::new())
}

pub fn serve(lib: Library, listener: TcpListener, workers: usize, max_requests: usize) -> io::Result<usize> {
    Ok(0)
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
