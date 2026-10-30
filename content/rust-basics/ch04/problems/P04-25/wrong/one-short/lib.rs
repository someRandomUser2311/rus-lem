pub fn stamp(mut text: String) -> String {
    text.push('#');
    text
}
pub fn relay_report(text: String, rounds: u32) -> (String, usize) {
    let mut text = text;
    for _ in 1..rounds {
        text = stamp(text);
    }
    let length = text.len();
    (text, length)
}
