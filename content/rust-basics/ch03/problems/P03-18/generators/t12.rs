fn main() {
    let mut g = Lcg(0x03000012);
    for _ in 0..300 {
        stdin(format!("{}\n", g.pick(1001)));
    }
}
