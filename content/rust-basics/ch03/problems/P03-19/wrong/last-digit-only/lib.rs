pub fn digit_sum(mut n: u32) -> u32 {
    let mut total = 0;
    while n > 9 {
        n /= 10;
    }
    total += n % 10;
    total
}
