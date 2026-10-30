pub fn measure(text: String) -> (String, usize) {
    let length = text.chars().count();
    (text, length)
}
