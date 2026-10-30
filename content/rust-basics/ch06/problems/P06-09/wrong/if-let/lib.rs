pub fn plus_one(value: Option<u32>) -> Option<u32> {
    if let Some(n) = value {
        n.checked_add(1)
    } else {
        None
    }
}
