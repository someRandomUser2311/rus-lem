pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for d in 2..n {
        if d > n / d {
            break;
        }
        if n % d == 0 {
            return false;
        }
    }
    true
}
