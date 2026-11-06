use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn parallel_map<F>(input: Vec<u32>, workers: usize, f: F) -> Vec<u64>
where
    F: Fn(u32) -> u64 + Send + Sync + 'static,
{
    let count = if input.is_empty() { 0 } else { workers.clamp(1, 8) };
    let queue: VecDeque<(usize, u32)> = input.into_iter().enumerate().collect();
    let queue = Arc::new(Mutex::new(queue));
    let f = Arc::new(f);
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..count {
        let queue = Arc::clone(&queue);
        let f = Arc::clone(&f);
        let tx = tx.clone();
        handles.push(thread::spawn(move || loop {
            let job = queue.lock().expect("the lock isn't poisoned").pop_front();
            match job {
                Some((i, v)) => tx.send((i, f(v))).expect("the receiver is alive"),
                None => break,
            }
        }));
    }
    drop(tx);
    let mut results: Vec<(usize, u64)> = rx.into_iter().collect();
    for h in handles {
        h.join().expect("a worker doesn't panic");
    }
    results.sort_by_key(|r| r.1);
    results.into_iter().map(|r| r.1).collect()
}
