pub fn revise(text: String) -> (String, String) {
    let original = String::from(text.as_str());
    let mut changed = text;
    changed.push_str(" [revised]");
    (original, changed)
}
