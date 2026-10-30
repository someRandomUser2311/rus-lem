pub fn remember(mut v: Vec<i32>, n: i32) -> (Option<i32>, Vec<i32>) {
    let first = v.get(0).copied();
    let mut copy = v.clone();
    copy.push(n);
    (first, copy)
}
