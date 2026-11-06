pub fn apply<F: Fn(i32) -> i32>(value: i32, f: F) -> i32 {
    f(value)
}
