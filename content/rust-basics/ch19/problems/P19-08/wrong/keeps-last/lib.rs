pub fn ends(v: &[i32]) -> Option<(&i32, &i32)> {
    match v {
        [] => None,
        [only] => Some((only, only)),
        [first, .., last] => Some((first, last)),
    }
}

pub fn middle(v: &[i32]) -> &[i32] {
    match v {
        [_, mid @ ..] => mid,
        _ => &[],
    }
}

pub fn describe(words: &[&str]) -> String {
    match words {
        [] => String::from("nothing"),
        [one] => format!("just {one}"),
        [a, b] => format!("{a} and {b}"),
        [first, rest @ ..] => format!("{first} and {} more", rest.len()),
    }
}
