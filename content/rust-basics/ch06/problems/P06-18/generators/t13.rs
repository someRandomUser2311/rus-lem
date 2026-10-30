fn main() {
    let mut g = Lcg(0x06000012);
    for _ in 0..300 {
        let old = g.next();
        let op = g.pick(3);
        let amount = g.next();
        let change = match op {
            0 => format!("Adjustment::Add({amount})"),
            1 => format!("Adjustment::Remove({amount})"),
            _ => "Adjustment::Clear".to_string(),
        };
        call(format!("step({old}, {change})"));
    }
}
