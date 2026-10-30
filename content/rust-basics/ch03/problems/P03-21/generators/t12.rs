fn main() {
    let mut g = Lcg(0x03000015);
    for _ in 0..300 {
        let a = g.next();
        let b = g.next();
        call(format!("gcd({a}, {b})"));
    }
}
