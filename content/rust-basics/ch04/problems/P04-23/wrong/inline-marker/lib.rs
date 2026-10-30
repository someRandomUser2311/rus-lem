pub fn marker_text() -> String {
    let marker = String::from("!");
    marker
}
pub fn decorate(text: &mut String) -> usize {
    let view = &*text;
    let old = view.len();
    text.push('!');
    old
}
