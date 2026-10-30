pub fn reverse_integer(mut n: u32) -> u64 {
    let mut result: u64 = 0;
    while n > 0 {
        result = result * 10 + u64::from(n % 10);
        n /= 10;
    }
    result
}
