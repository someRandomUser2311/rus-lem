pub fn reverse_integer(mut n: u32) -> u64 {
    let mut result = 0_u32;
    while n > 0 {
        result = result.wrapping_mul(10).wrapping_add(n % 10);
        n /= 10;
    }
    result as u64
}
