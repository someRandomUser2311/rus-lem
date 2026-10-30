pub fn read(v: &[i32], index: usize, fallback: i32) -> i32 {
    let x = v.get(index).copied().unwrap_or(0);
    if x == 0 { fallback } else { x }
}
