pub fn stable_unique<T: PartialEq>(values: &[T]) -> Vec<&T> {
    let mut out: Vec<&T> = Vec::new();
    for i in 0..values.len() {
        let value = &values[i];
        let mut seen = false;
        for kept in &values[i + 1..] {
            if kept == value {
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
