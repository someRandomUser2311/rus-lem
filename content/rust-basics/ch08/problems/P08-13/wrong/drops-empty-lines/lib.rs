pub fn records(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let s = line.trim();
        let s = s.strip_prefix('#').unwrap_or(s).trim();
        if !s.is_empty() {
            out.push(s.to_string());
        }
    }
    out
}
