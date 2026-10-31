pub fn choose<'a>(a: &'a str, b: &'a str, first: bool) -> &'a str {
    if first { b } else { a }
}
