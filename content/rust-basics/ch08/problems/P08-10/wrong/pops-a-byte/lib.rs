pub fn pop_scalar(mut text: String) -> (String, Option<char>) {
    let c = if text.is_empty() { None } else { Some(text.remove(text.len() - 1)) };
    (text, c)
}
