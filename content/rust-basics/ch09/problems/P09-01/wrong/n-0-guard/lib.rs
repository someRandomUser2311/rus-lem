pub fn positive(n: i32) -> u32 {
    if n < 0 {
        panic!("count must be positive, got {n}");
    }
    n as u32
}
