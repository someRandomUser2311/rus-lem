pub trait Textual {
    fn text(&self) -> &str;
}

pub struct Entry<T> {
    pub key: T,
    pub text: String,
}

impl<T> Textual for Entry<T> {
    fn text(&self) -> &str {
        &self.text
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Entry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.text)
    }
}

pub fn query<'a, T>(
    items: &'a [T],
    query: &str,
    announcement: &impl std::fmt::Display,
) -> (Vec<&'a T>, String)
where
    T: Textual + std::fmt::Display,
{
    let wanted = query.to_lowercase();
    let mut found = Vec::new();
    let mut lines = Vec::new();
    for item in items {
        let text = item.text().to_ascii_lowercase();
        let mut matches = true;
        for word in wanted.split_whitespace() {
            if !text.contains(word) {
                matches = false;
                break;
            }
        }
        if matches {
            found.push(item);
            lines.push(item.to_string());
        }
    }
    (found, format!("{}\n{}", announcement, lines.join("\n")))
}
