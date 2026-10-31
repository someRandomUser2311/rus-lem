pub fn owned_static_label<T: std::fmt::Display + 'static>(value: T) -> String {
    format!("{value:>3}")
}
