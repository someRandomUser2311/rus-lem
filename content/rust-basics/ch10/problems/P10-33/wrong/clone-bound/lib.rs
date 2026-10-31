pub fn owned_static_label<T: std::fmt::Display + Clone + 'static>(value: T) -> String {
    format!("{value}")
}
