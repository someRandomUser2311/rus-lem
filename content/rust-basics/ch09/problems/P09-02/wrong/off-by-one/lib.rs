pub fn strict(v: &[i32], index: usize) -> i32 {
    if index > v.len() {
        panic!("index {index} is out of range for length {}", v.len());
    }
    v[index]
}
