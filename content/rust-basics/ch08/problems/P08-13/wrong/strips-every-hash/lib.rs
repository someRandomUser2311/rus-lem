pub fn records(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let s = line.trim();
        let s = s.trim_start_matches('#').trim();
        out.push(s.to_string());
    }
    out
}
