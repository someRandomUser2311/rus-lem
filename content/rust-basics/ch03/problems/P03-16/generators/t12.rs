fn main() {
    let mut g = Lcg(0x03000010);
    for _ in 0..300 {
        call(format!("sum_range({})", g.pick(100001)));
    }
}
