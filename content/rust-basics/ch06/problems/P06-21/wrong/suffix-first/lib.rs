pub fn decorate(label: Option<String>, suffix: &str) -> Option<String> {
    let Some(mut text) = label else {
        return None;
    };
    text.insert_str(0, suffix);
    Some(text)
}
