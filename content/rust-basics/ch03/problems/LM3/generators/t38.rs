fn main() {
    let mut g = Lcg(0x0003b004);
    for _ in 0..300 {
        call(format!("format_cents({})", g.next()));
    }
}
