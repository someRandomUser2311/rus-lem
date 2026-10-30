pub fn partition(v: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut negative = Vec::new();
    let mut other = Vec::new();
    for n in v {
        if *n <= 0 {
            negative.push(*n);
        } else {
            other.push(*n);
        }
    }
    (negative, other)
}
