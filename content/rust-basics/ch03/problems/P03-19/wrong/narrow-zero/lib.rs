pub fn digit_sum(mut n: u32) -> u32 {
    let mut total = n % 10;
    n /= 10;
    while n > 9 {
        total += n % 10;
        n /= 10;
    }
    total
}
