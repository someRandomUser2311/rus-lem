pub fn decimal(c: char) -> Option<u32> {
    match c {
        '1'..='9' => c.to_digit(10),
        _ => None,
    }
}
