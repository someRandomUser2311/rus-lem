pub fn counter(label: String, start: i32) -> impl FnMut(i32) -> String {
    let mut total = start;
    move |delta| {
        total += delta;
        format!("{label}: {total}")
    }
}
