use std::collections::{HashMap, VecDeque};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn add_words(map: &mut HashMap<String, u64>, text: &str) {
    for word in text.split_whitespace() {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }
}

fn sorted(map: HashMap<String, u64>) -> Vec<(String, u64)> {
    let mut pairs: Vec<(String, u64)> = map.into_iter().collect();
    pairs.sort();
    pairs
}

pub fn count_one(text: &str) -> Vec<(String, u64)> {
    let mut map = HashMap::new();
    add_words(&mut map, text);
    sorted(map)
}

pub fn count_all(texts: Vec<String>, workers: usize) -> Vec<(String, u64)> {
    let count = workers.clamp(1, 8).min(texts.len());
    let queue: VecDeque<String> = texts.into_iter().collect();
    let queue = Arc::new(Mutex::new(queue));
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..count {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let mut local = HashMap::new();
            loop {
                let job = queue.lock().expect("the lock isn't poisoned").pop_front();
                match job {
                    Some(text) => add_words(&mut local, &text),
                    None => break,
                }
            }
            tx.send(local).expect("the receiver is alive");
        }));
    }
    drop(tx);
    let mut total: HashMap<String, u64> = HashMap::new();
    for map in rx {
        for (word, c) in map {
            *total.entry(word).or_insert(0) += c;
        }
    }
    for h in handles {
        h.join().expect("a worker doesn't panic");
    }
    sorted(total)
}

pub fn top(texts: Vec<String>, workers: usize, n: usize) -> Vec<(String, u64)> {
    let mut pairs = count_all(texts, workers);
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs.into_iter().take(n).collect()
}
