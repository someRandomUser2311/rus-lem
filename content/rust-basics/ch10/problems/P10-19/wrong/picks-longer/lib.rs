pub fn choose<'a>(a: &'a str, b: &'a str, first: bool) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
