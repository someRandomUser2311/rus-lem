pub fn optional_total(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    Some(a? + b?)
}
