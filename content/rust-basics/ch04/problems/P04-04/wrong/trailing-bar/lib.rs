pub fn relay(message: String, hops: u32) -> String {
    let original = message;
    let mut trail = String::new();
    for _ in 0..hops {
        trail = format!("{trail}{original}|");
    }
    format!("{trail}{original}|")
}
