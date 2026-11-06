pub fn twice<F: FnMut(i32) -> i32>(value: i32, mut f: F) -> (i32, i32) {
    let first = f(value);
    let second = f(value);
    (first, second)
}
