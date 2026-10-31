pub fn strict(v: &[i32], index: usize) -> i32 {
    *v.get(index).unwrap()
}
