pub fn counts(text: &str) -> (usize, usize, usize) {
    let mut scalars = 0;
    let mut ascii = 0;
    for c in text.chars() {
        scalars += 1;
        if c.is_ascii() {
            ascii += 1;
        }
    }
    (text.len(), text.len(), ascii)
}
