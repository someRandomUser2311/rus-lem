/// Starts from `Member { id: 7, name: " unchanged ", premium, borrowed }` and runs each
/// operation (`b` record_borrow, `r` record_return), logging its result and the count.
fn drive(premium: bool, borrowed: u8, ops: &str) -> String {
    let mut m = Member {
        id: 7,
        name: " unchanged ".to_string(),
        premium,
        borrowed,
    };
    let mut log = String::new();
    for op in ops.chars() {
        let result = if op == 'b' {
            format!("{:?}", m.record_borrow())
        } else {
            m.record_return();
            "()".to_string()
        };
        log.push_str(&format!("{op} -> {result} {} {}\n", m.borrowed, m.limit()));
    }
    log.push_str(&format!("{:?}", m));
    log
}
