pub fn reverse_integer(mut n: u32) -> u64 {
    let mut result = 0_u64;
    while n > 0 {
        result = result * 10 + (n % 10) as u64;
        n /= 10;
    }
    result
}
