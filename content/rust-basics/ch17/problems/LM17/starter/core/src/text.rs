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

/// The title used for sorting: without a leading "The ", "A " or "An ".
///
/// # Examples
///
/// ```
/// use library::text::sort_key;
///
/// assert_eq!(sort_key("The Hobbit"), "Hobbit");
/// assert_eq!(sort_key("Dune"), "Dune");
/// ```
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
