pub fn collatz_report(mut n: u64, step_limit: u32) -> (u64, u32, bool) {
    let mut steps = 0;
    if n == 0 {
        return (0, 0, false);
    }
    while n != 1 && steps < step_limit {
        if n % 2 == 0 {
            n /= 2;
        } else if n > (u64::MAX - 1) / 3 {
            return (n, steps, false);
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    (n, steps, n == 1)
}
