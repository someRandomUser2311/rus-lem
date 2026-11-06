pub fn swap((a, b): (String, String)) -> (String, String) {
    (a, b)
}

pub fn label(&(x, y): &(i32, i32)) -> String {
    format!("({x}, {y})")
}

pub fn join_counts(pairs: Vec<(String, u32)>) -> (String, u32) {
    let mut names = Vec::new();
    let mut total: u32 = 0;
    for (name, count) in pairs {
        names.push(name);
        total = total.saturating_add(count);
    }
    (names.join(","), total)
}
