pub fn announced_choice<'a, T: std::fmt::Display>(
    a: &'a str,
    b: &'a str,
    announcement: T,
) -> (&'a str, String) {
    let chosen = if a.len() >= b.len() {
        a
    } else {
        b
    };
    (chosen, format!("Choosing: {announcement}"))
}
