pub fn plus_one(value: Option<u32>) -> Option<u32> {
    match value {
        Some(n) => Some(n.wrapping_add(1)),
        None => None,
    }
}
