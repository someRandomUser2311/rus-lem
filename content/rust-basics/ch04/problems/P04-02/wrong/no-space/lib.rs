pub fn countdown(n: u32) -> String {
    let mut text = String::new();
    for k in (1..=n).rev() {
        text = format!("{text}{k}...");
    }
    text.push_str("Liftoff!");
    text
}
