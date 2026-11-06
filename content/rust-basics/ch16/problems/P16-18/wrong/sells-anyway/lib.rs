use std::sync::{Arc, Mutex};
use std::thread;

pub struct Shop {
    pub stock: u32,
    pub handled: usize,
    pub sold: usize,
}

pub fn reserve(stock: u32, requests: usize, workers: usize) -> (u32, usize) {
    let shop = Arc::new(Mutex::new(Shop {
        stock,
        handled: 0,
        sold: 0,
    }));
    let count = if requests == 0 { 0 } else { workers.clamp(1, 8) };
    let mut handles = Vec::new();
    for _ in 0..count {
        let shop = Arc::clone(&shop);
        handles.push(thread::spawn(move || loop {
            let mut s = shop.lock().expect("the lock isn't poisoned");
            if s.handled == requests {
                break;
            }
            s.handled += 1;
            s.stock = s.stock.saturating_sub(1);
            s.sold += 1;
        }));
    }
    for h in handles {
        h.join().expect("a worker doesn't panic");
    }
    let s = shop.lock().expect("the lock isn't poisoned");
    (s.stock, s.sold)
}
