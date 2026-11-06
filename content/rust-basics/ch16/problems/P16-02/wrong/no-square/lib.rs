use std::thread;

pub fn squares(n: u32) -> Vec<u64> {
    let handles: Vec<_> = (0..n)
        .map(|i| thread::spawn(move || i as u64))
        .collect();
    handles
        .into_iter()
        .map(|h| h.join().expect("the worker doesn't panic"))
        .collect()
}
