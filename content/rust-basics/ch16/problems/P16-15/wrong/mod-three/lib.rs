use std::sync::{Arc, Mutex};
use std::thread;

pub fn histogram(values: Vec<u8>) -> [u32; 4] {
    let bins = Arc::new(Mutex::new([0u32; 4]));
    let mut handles = Vec::new();
    for chunk in values.chunks(8) {
        let owned = chunk.to_vec();
        let bins = Arc::clone(&bins);
        handles.push(thread::spawn(move || {
            let mut local = [0u32; 4];
            for v in owned {
                local[(v % 3) as usize] += 1;
            }
            let mut shared = bins.lock().expect("the lock isn't poisoned");
            for i in 0..4 {
                shared[i] += local[i];
            }
        }));
    }
    for h in handles {
        h.join().expect("a worker doesn't panic");
    }
    let out = *bins.lock().expect("the lock isn't poisoned");
    out
}
