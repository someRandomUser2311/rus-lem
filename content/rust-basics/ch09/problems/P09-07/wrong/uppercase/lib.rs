pub fn hex_digit(n: u32) -> char {
    char::from_digit(n, 16).expect("n must be below 16").to_ascii_uppercase()
}
