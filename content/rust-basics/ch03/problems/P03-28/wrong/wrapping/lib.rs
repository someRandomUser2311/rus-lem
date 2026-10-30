pub fn collatz_report(mut n: u64, step_limit: u32) -> (u64, u32, bool) {
    let mut steps = 0;
    loop {
        if n == 0 {
            break (0, 0, false);
        }
        if n == 1 {
            break (n, steps, true);
        }
        if steps == step_limit {
            break (n, steps, false);
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            let (tripled, _) = n.overflowing_mul(3);
            let (next, _) = tripled.overflowing_add(1);
            n = next;
        }
        steps += 1;
    }
}
