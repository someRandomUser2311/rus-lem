fn main() {
    let mut g = Lcg(0x0003b001);
    for _ in 0..300 {
        let day = g.pick(4294967268);
        let premium = g.pick(2) == 1;
        call(format!("due_day({day}, {premium})"));
    }
}
