use std::sync::Mutex;
use std::thread;

pub fn withdrawals(balance: u32, count: usize) -> (u32, usize) {
    let m = Mutex::new(balance);
    let successes = thread::scope(|s| {
        let handles: Vec<_> = (0..count)
            .map(|_| {
                s.spawn(|| {
                    let mut b = m.lock().expect("the lock isn't poisoned");
                    if *b >= 0 {
                        *b = b.saturating_sub(1);
                        true
                    } else {
                        false
                    }
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("a worker doesn't panic"))
            .filter(|ok| *ok)
            .count()
    });
    (m.into_inner().expect("the lock isn't poisoned"), successes)
}
