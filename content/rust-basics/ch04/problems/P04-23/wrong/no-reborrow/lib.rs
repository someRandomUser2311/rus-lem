pub fn marker_text() -> String {
    let marker = String::from("!");
    marker
}
pub fn decorate(text: &mut String) -> usize {
    let old = text.len();
    let marker = marker_text();
    text.push_str(&marker);
    old
}
