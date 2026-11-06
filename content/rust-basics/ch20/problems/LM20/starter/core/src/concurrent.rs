use std::sync::mpsc;
use std::thread;

use crate::Library;
use crate::cli::load_catalog;

fn assert_shareable<T: Send + Sync>() {}

pub fn library_is_shareable() -> bool {
    assert_shareable::<Library>();
    true
}

pub fn search_many(library: &Library, queries: Vec<String>, workers: usize) -> Vec<Vec<u32>> {
    if queries.is_empty() {
        return Vec::new();
    }
    let count = workers.clamp(1, 8).min(queries.len());
    let size = queries.len().div_ceil(count);
    thread::scope(|s| {
        let handles: Vec<_> = queries
            .chunks(size)
            .map(|chunk| {
                s.spawn(move || {
                    chunk
                        .iter()
                        .map(|q| library.find(q).iter().map(|b| b.id).collect())
                        .collect::<Vec<Vec<u32>>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("a search doesn't panic"))
            .collect()
    })
}

pub fn load_many(catalogs: Vec<String>) -> Vec<Result<usize, String>> {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for (i, text) in catalogs.into_iter().enumerate() {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let result = load_catalog(&text).map(|l| l.find("").len());
            tx.send((i, result)).expect("the receiver is alive");
        }));
    }
    drop(tx);
    let mut results: Vec<(usize, Result<usize, String>)> = rx.into_iter().collect();
    for h in handles {
        h.join().expect("a loader doesn't panic");
    }
    results.sort_by_key(|r| r.0);
    results.into_iter().map(|r| r.1).collect()
}
