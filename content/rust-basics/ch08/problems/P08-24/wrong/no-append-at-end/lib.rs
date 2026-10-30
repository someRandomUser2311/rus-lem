pub fn insert_scalar(text: &str, position: usize, ch: char) -> Option<String> {
    let mut count = 0;
    for (i, _) in text.char_indices() {
        if count == position {
            let mut out = text.to_string();
            out.insert(i, ch);
            return Some(out);
        }
        count += 1;
    }
    None
}
