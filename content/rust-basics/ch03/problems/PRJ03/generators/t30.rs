fn main() {
    let mut g = Lcg(0x0003a002);
    for _ in 0..300 {
        let a = g.next();
        let b = g.next();
        call(format!("gcd({a}, {b})"));
    }
}
