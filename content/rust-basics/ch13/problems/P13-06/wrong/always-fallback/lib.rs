pub fn choose<F: FnOnce() -> String>(value: Option<String>, fallback: F) -> String {
    match value {
        Some(_) => fallback(),
        None => fallback(),
    }
}
