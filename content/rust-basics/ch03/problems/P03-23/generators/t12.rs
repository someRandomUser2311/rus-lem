fn main() {
    let mut g = Lcg(0x03000017);
    for _ in 0..300 {
        call(format!("is_prime({})", g.pick(1000001)));
    }
}
