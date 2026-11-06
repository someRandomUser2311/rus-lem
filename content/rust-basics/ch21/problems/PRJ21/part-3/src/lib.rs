use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "a pool needs at least one worker");
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            sender.send(Box::new(f)).expect("the workers are running");
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().expect("not poisoned").recv();
                match message {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    pub fn shutdown(mut self) -> Vec<usize> {
        self.stop()
    }

    fn stop(&mut self) -> Vec<usize> {
        drop(self.sender.take());
        let mut joined = Vec::new();
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
                joined.push(worker.id);
            }
        }
        joined
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}

pub fn respond(request_line: &str, slow: Duration) -> (u16, &'static str, String) {
    let words: Vec<&str> = request_line.split(' ').collect();
    match words.as_slice() {
        ["GET", "/", "HTTP/1.1"] => (200, "OK", String::from("Hello!")),
        ["GET", "/sleep", "HTTP/1.1"] => {
            thread::sleep(slow);
            (200, "OK", String::from("Slept"))
        }
        ["GET", path, "HTTP/1.1"] if path.starts_with("/echo/") => (200, "OK", path[6..].to_string()),
        ["GET", _, "HTTP/1.1"] => (404, "Not Found", String::from("Not found")),
        _ => (400, "Bad Request", String::from("Bad request")),
    }
}

pub fn format_response(status: u16, reason: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )
}

fn handle(mut stream: TcpStream, slow: Duration) -> io::Result<()> {
    let mut line = String::new();
    let n = {
        let mut reader = BufReader::new(&mut stream);
        let n = (&mut reader).take(8192).read_line(&mut line);
        let _ = io::copy(&mut (&mut reader).take(1 << 20), &mut io::sink());
        n
    };
    let (status, reason, body) = match n {
        Ok(_) => respond(line.trim_end_matches(['\r', '\n']), slow),
        Err(_) => (400, "Bad Request", String::from("Bad request")),
    };
    stream.write_all(format_response(status, reason, &body).as_bytes())
}

pub fn serve(listener: TcpListener, workers: usize, max_requests: usize, slow: Duration) -> io::Result<usize> {
    let pool = ThreadPool::new(workers);
    let mut handled = 0;
    for stream in listener.incoming().take(max_requests) {
        let stream = stream?;
        pool.execute(move || {
            let _ = handle(stream, slow);
        });
        handled += 1;
    }
    drop(pool);
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
