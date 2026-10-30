pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a == 0 {
        return b;
    }
    while b != 0 {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}
