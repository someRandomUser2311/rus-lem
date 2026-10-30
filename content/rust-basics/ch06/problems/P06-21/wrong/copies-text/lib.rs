pub fn decorate(label: Option<String>, suffix: &str) -> Option<String> {
    let Some(mut text) = label.clone() else {
        return None;
    };
    text.push_str(suffix);
    Some(text)
}
