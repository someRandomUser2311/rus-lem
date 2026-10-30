pub fn wrap(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('[');
    out.push_str(text);
    out
}
