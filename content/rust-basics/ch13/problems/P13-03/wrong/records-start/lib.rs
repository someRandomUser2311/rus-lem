pub fn running(start: i32, deltas: &[i32]) -> Vec<i64> {
    let mut total = start as i64;
    let mut bump = |n: i32| {
        let before = total;
        total += n as i64;
        before
    };
    let mut out = Vec::new();
    for n in deltas {
        out.push(bump(*n));
    }
    out
}
