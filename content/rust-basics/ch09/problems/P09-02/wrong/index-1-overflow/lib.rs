pub fn strict(v: &[i32], index: usize) -> i32 {
    if index + 1 > v.len() {
        panic!("index {index} is out of range for length {}", v.len());
    }
    v[index]
}
