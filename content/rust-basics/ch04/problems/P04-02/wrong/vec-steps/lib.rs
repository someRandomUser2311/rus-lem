pub fn countdown(n: u32) -> String {
    let steps: Vec<u32> = (1..=n).rev().collect();
    let mut text = String::new();
    for k in steps {
        text = format!("{text}{k}... ");
    }
    text.push_str("Liftoff!");
    text
}
