use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

static BATCH: Mutex<Vec<i64>> = Mutex::new(Vec::new());
static RUNS: AtomicU64 = AtomicU64::new(0);

pub fn run_batch(values: &[i64]) -> i64 {
    let mut buf = BATCH.lock().unwrap_or_else(|e| e.into_inner());
    buf.extend_from_slice(values);
    let sum = buf.iter().fold(0i64, |t, &v| t.saturating_add(v));
    RUNS.fetch_add(1, Ordering::SeqCst);
    sum
}

pub fn batches_run() -> u64 {
    RUNS.load(Ordering::SeqCst)
}
