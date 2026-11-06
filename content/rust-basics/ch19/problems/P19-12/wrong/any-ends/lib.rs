pub fn window(v: &[i32]) -> Option<&[i32]> {
    match v {
        [_, mid @ .., _] => Some(mid),
        _ => None,
    }
}

pub fn shape(v: &[i32]) -> String {
    match v {
        [] => String::from("empty"),
        [x] if *x < 0 => String::from("one negative"),
        [_] => String::from("one"),
        [a, b] if a == b => format!("pair of {a}"),
        all @ [first, .., last] if first == last && all.len() >= 3 => {
            format!("loop of {}", all.len())
        }
        [first, ..] => format!("starts with {first}"),
    }
}
