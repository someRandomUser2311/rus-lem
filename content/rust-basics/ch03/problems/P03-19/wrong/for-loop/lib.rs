pub fn digit_sum(mut n: u32) -> u32 {
    let mut total = 0;
    for _ in 0..10 {
        total += n % 10;
        n /= 10;
    }
    total
}
