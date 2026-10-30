pub fn trimmed(text: &str) -> &str {
    text.trim_matches(|c: char| c == ' ' || c == '\t' || c == '\n')
}
