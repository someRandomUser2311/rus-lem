pub fn compare_display<T>(a: &T, b: &T) -> String
where
    T: std::fmt::Display + PartialEq,
{
    if format!("{a}") == format!("{b}") {
        format!("equal: {a}")
    } else {
        format!("different: {a} / {b}")
    }
}
