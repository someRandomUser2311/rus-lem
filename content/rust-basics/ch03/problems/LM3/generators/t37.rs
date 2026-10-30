fn main() {
    let mut g = Lcg(0x0003b003);
    for _ in 0..300 {
        call(format!("late_fee_cents({})", g.next()));
    }
}
