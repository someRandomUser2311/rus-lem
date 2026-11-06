pub fn count_some(values: &[Option<String>]) -> usize {
    let mut n = 0;
    for v in values {
        if let _ = v {
            n += 1;
        }
    }
    n
}

pub fn first_and_last(t: (i32, i32, i32, i32, i32)) -> (i32, i32) {
    let (first, .., last) = t;
    (first, last)
}

pub fn second(t: (u8, u8, u8)) -> u8 {
    let (_, x, _) = t;
    x
}
