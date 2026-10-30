pub fn fallback(value: Option<i32>, default: i32) -> i32 {
    value.unwrap_or(0)
}
