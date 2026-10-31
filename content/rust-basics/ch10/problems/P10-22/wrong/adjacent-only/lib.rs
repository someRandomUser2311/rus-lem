pub fn stable_unique<T: PartialEq>(values: &[T]) -> Vec<&T> {
    let mut out: Vec<&T> = Vec::new();
    for value in values {
        let mut seen = false;
        if let Some(kept) = out.last() {
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
