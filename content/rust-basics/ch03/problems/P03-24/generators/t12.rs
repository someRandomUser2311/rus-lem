fn main() {
    let mut g = Lcg(0x03000018);
    for _ in 0..300 {
        let start = g.pick(256);
        let steps = g.pick(100001);
        call(format!("capped_counter({start}, {steps})"));
    }
}
