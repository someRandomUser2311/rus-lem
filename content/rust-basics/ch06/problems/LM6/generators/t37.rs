fn main() {
    let mut g = Lcg(0x06020000);
    for i in 0..300 {
        let member_id = g.next();
        let due = g.next();
        let raw_day = g.next();
        call(format!("lm6_case({i}, {member_id}, {due}, {raw_day})"));
    }
}
