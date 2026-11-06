use std::sync::atomic::{AtomicU64, Ordering};

static ISSUED: AtomicU64 = AtomicU64::new(0);

pub fn next_ticket() -> u64 {
    ISSUED.fetch_add(2, Ordering::SeqCst) + 2
}

pub fn issued() -> u64 {
    ISSUED.load(Ordering::SeqCst)
}
