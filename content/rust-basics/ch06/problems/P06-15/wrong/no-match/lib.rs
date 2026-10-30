pub fn decimal(c: char) -> Option<u32> {
    c.to_digit(10)
}
