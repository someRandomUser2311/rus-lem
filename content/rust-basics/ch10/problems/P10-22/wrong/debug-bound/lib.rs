pub fn stable_unique<T: PartialEq + std::fmt::Debug>(values: &[T]) -> Vec<&T> {
    let mut out: Vec<&T> = Vec::new();
    for value in values {
        let mut seen = false;
        for kept in &out {
            if *kept == value {
                seen = true;
                break;
            }
        }
        if !seen {
            out.push(value);
        }
    }
    out
}
