pub fn relay(message: String, hops: u32) -> String {
    let original = message;
    let mut trail = String::new();
    for _ in 0..hops {
        let carried = original.clone();
        trail = format!("{trail}{carried}|");
    }
    format!("{trail}{original}")
}
