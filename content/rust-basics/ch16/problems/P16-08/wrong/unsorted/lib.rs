use std::sync::mpsc;
use std::thread;

pub fn group_totals(groups: Vec<Vec<u32>>) -> Vec<u64> {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for (i, group) in groups.into_iter().enumerate() {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let total: u64 = group.iter().map(|n| *n as u64).sum();
            tx.send((i, total)).expect("the receiver is alive");
        }));
    }
    drop(tx);
    let mut got: Vec<(usize, u64)> = rx.into_iter().collect();
    for h in handles {
        h.join().expect("a worker doesn't panic");
    }
    got.sort_by_key(|m| m.1);
    got.into_iter().map(|m| m.1).collect()
}
