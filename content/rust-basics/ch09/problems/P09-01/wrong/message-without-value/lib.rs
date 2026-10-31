pub fn positive(n: i32) -> u32 {
    if n <= 0 {
        panic!("count must be positive");
    }
    n as u32
}
