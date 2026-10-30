pub fn read(v: &[i32], index: usize, fallback: i32) -> i32 {
    if index < v.len() { v[index] } else { fallback }
}
