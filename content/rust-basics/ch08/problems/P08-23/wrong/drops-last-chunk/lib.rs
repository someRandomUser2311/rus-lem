pub fn chunks(text: &str, size: usize) -> Vec<String> {
    let mut out = Vec::new();
    if size == 0 {
        return out;
    }
    let mut start = 0;
    let mut count = 0;
    for (i, _) in text.char_indices() {
        if count == size {
            out.push(text[start..i].to_string());
            start = i;
            count = 0;
        }
        count += 1;
    }
    out
}
