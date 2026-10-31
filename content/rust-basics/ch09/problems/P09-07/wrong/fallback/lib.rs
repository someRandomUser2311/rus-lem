pub fn hex_digit(n: u32) -> char {
    char::from_digit(n, 16).unwrap_or('?')
}
