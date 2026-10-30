fn main() {
    let mut g = Lcg(0x0003a003);
    for _ in 0..300 {
        call(format!("is_prime({})", g.pick(1000001)));
    }
}
