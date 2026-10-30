fn main() {
    let mut g = Lcg(0x05000015);
    for _ in 0..300 {
        let count = g.pick(65);
        let mut ops = String::new();
        for _ in 0..count {
            let op = g.pick(3);
            let a = g.pick(11);
            let b = g.pick(11);
            let id = g.next();
            let text = match op {
                0 => format!("r{a},{id}"),
                1 => format!("c{a}"),
                _ => format!("t{a},{b}"),
            };
            if !ops.is_empty() {
                ops.push('|');
            }
            ops.push_str(&text);
        }
        call(format!("drive({ops:?})"));
    }
}
