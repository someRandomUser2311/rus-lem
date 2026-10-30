fn main() {
    let mut g = Lcg(0x06000019);
    for _ in 0..300 {
        let old = g.next();
        let op = g.pick(3);
        let amount = g.next();
        let action = match op {
            0 => format!("Purchase::Coin({amount})"),
            1 => format!("Purchase::Buy {{ price: {amount} }}"),
            _ => "Purchase::Refund".to_string(),
        };
        call(format!("step({old}, {action})"));
    }
}
