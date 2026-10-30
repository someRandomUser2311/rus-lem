pub fn first_digit(text: &str, radix: u32) -> Option<char> {
    let Some(c) = text.chars().next() else {
        return None;
    };
    let Some(n) = c.to_digit(10) else {
        return None;
    };
    char::from_digit(n, radix)
}
