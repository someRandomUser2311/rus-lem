pub fn normalize_title(title: &str) -> String {
    let mut out = String::new();
    for word in title.split_whitespace() {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}
pub fn sort_key(title: &str) -> &str {
    if title.starts_with("The ") {
        &title[4..]
    } else if title.starts_with("A ") {
        &title[2..]
    } else if title.starts_with("An ") {
        &title[3..]
    } else {
        title
    }
}
pub fn title_matches(title: &str, query: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}
