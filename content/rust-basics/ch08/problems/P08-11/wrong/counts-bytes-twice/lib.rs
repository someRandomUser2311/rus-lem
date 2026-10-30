pub fn counts(text: &str) -> (usize, usize) {
    let mut scalars = 0;
    for _c in text.bytes() {
        scalars += 1;
    }
    (text.len(), scalars)
}
