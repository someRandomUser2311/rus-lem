pub fn collatz_report(mut n: u64, step_limit: u32) -> (u64, u32, bool) {
    let mut steps = 0;
    loop {
        if n == 0 {
            break (0, 0, false);
        }
        if steps == step_limit {
            break (n, steps, false);
        }
        if n == 1 {
            break (n, steps, true);
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            let (tripled, over_mul) = n.overflowing_mul(3);
            let (next, over_add) = tripled.overflowing_add(1);
            if over_mul || over_add {
                break (n, steps, false);
            }
            n = next;
        }
        steps += 1;
    }
}
