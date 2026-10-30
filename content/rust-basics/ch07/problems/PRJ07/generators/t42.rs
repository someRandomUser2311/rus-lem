fn main() {
    let mut g = Lcg(0x06010000);
    for _ in 0..300 {
        let stock = [g.pick(6), g.pick(6), g.pick(6)];
        let credit = g.next();
        let mut ops = String::new();
        for _ in 0..20 {
            let op = g.pick(3);
            let arg = g.pick(4);
            let code = match op {
                0 => format!("i{arg}"),
                1 => format!("b{}", arg % 3),
                _ => "r".to_string(),
            };
            if !ops.is_empty() {
                ops.push('|');
            }
            ops.push_str(&code);
        }
        call(format!("drive({stock:?}, {credit}, {ops:?})"));
    }
}
