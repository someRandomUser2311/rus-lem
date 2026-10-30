pub fn remember(mut v: Vec<i32>, n: i32) -> (Option<i32>, Vec<i32>) {
    v.push(n);
    let first = v.get(0).copied();
    (first, v)
}
