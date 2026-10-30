fn main() {
    let mut g = Lcg(0x0003b002);
    for _ in 0..300 {
        let due = g.next();
        let ret = g.next();
        call(format!("days_late({due}, {ret})"));
    }
}
