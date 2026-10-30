pub fn first_digit(text: &str, radix: u32) -> Option<char> {
    let c = match text.chars().next() {
        Some(c) => c,
        None => return None,
    };
    let Some(n) = c.to_digit(radix) else {
        return None;
    };
    char::from_digit(n, radix)
}
