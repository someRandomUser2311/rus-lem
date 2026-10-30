pub fn at_or(xs: &[i32], index: usize, default: i32) -> i32 {
    *xs.get(index + 1).unwrap_or(&default)
}
pub fn ends(xs: &[i32], default: i32) -> (i32, i32) {
    (
        *xs.first().unwrap_or(&default),
        *xs.last().unwrap_or(&default),
    )
}
