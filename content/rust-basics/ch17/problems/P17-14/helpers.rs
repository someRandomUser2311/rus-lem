struct Counter(std::sync::atomic::AtomicUsize);
impl std::task::Wake for Counter {
    fn wake(self: std::sync::Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}
