fn main() {
    let mut g = Lcg(0x0300001A);
    for _ in 0..300 {
        let year = 1 + g.pick(9999);
        let month = g.pick(14);
        call(format!("calendar({year}, {month})"));
    }
}
