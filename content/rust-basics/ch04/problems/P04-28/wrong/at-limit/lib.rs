pub fn mask_words(text: &str, limit: usize) -> String {
    let mut out = String::new();
    for word in text.split_whitespace() {
        if !out.is_empty() {
            out.push(' ');
        }
        let n = word.chars().count();
        if n >= limit {
            for _ in 0..n {
                out.push('*');
            }
        } else {
            out.push_str(word);
        }
    }
    out
}
