pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.chars().count() >= b.chars().count() { a } else { b }
}
