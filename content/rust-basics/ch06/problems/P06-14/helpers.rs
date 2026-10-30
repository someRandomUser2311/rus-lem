/// The first word and its byte offset in `text`, so the suite checks the slice's position.
fn probe(text: &str) -> Option<(String, usize)> {
    let word = first_word(text)?;
    Some((word.to_string(), word.as_ptr() as usize - text.as_ptr() as usize))
}
