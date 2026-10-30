pub fn total(v: &[i32]) -> i64 {
    let mut total = 0i32;
    for n in v {
        total = total.wrapping_add(*n);
    }
    total as i64
}
