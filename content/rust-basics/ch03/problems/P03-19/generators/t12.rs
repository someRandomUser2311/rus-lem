fn main() {
    let mut g = Lcg(0x03000013);
    for _ in 0..300 {
        call(format!("digit_sum({})", g.next()));
    }
}
