fn main() {
    let mut g = Lcg(0x06000011);
    for _ in 0..300 {
        let len = g.pick(41);
        let mut items = Vec::new();
        for _ in 0..len {
            if g.pick(3) != 0 {
                items.push(format!("Some({})", g.pick(2001) as i32 - 1000));
            } else {
                items.push("None".to_string());
            }
        }
        call(format!("maximum(&[{}])", items.join(", ")));
    }
}
