fn main() {
    let mut g = Lcg(0x0003a001);
    for _ in 0..300 {
        call(format!("digit_sum({})", g.next()));
    }
}
