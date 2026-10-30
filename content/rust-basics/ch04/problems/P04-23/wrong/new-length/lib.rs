pub fn marker_text() -> String {
    let marker = String::from("!");
    marker
}
pub fn decorate(text: &mut String) -> usize {
    let marker = marker_text();
    text.push_str(&marker);
    let view = &*text;
    view.len()
}
