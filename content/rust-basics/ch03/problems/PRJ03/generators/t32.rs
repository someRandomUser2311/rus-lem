fn main() {
    let mut g = Lcg(0x0003a004);
    for _ in 0..300 {
        call(format!("fibonacci({})", g.pick(47)));
    }
}
