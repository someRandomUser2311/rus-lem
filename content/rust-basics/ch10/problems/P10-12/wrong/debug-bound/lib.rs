pub fn compare_display<T>(a: &T, b: &T) -> String
where
    T: std::fmt::Display + PartialEq + std::fmt::Debug,
{
    if a == b {
        format!("equal: {a}")
    } else {
        format!("different: {a} / {b}")
    }
}
