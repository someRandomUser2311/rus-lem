pub fn revise(text: String) -> (String, String) {
    let original = text.clone();
    let mut changed = text;
    changed.push_str(" [revised]");
    let _ = original;
    let again = changed.clone();
    (again, changed)
}
