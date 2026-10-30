pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    loop {
        if b == 0 {
            break a;
        }
        let r = a % b;
        a = b;
        b = r;
    }
}
