use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn reverse_all(texts: Vec<String>, workers: usize) -> Vec<String> {
    let count = workers.clamp(1, 8).min(texts.len());
    let queue: VecDeque<(usize, String)> = texts.into_iter().enumerate().collect();
    let queue = Arc::new(Mutex::new(queue));
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..count {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        handles.push(thread::spawn(move || loop {
            let job = queue.lock().expect("the lock isn't poisoned").pop_front();
            match job {
                Some((i, text)) => {
                    let reversed: String = text;
                    tx.send((i, reversed)).expect("the receiver is alive");
                }
                None => break,
            }
        }));
    }
    drop(tx);
    let mut results: Vec<(usize, String)> = rx.into_iter().collect();
    for h in handles {
        h.join().expect("a worker doesn't panic");
    }
    results.sort_by_key(|r| r.0);
    results.into_iter().map(|r| r.1).collect()
}
