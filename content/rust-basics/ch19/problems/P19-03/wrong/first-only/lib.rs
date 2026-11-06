pub fn split_name(full: &str) -> (String, String) {
    let full = full.trim();
    match full.split_once(' ') {
        Some((first, rest)) => (first.to_string(), rest.trim().to_string()),
        None => (full.to_string(), String::new()),
    }
}

pub fn initials(full: &str) -> String {
    let (first, rest) = split_name(full);
    let mut out = String::new();
    let _ = rest;
    for part in [first] {
        if let Some(c) = part.chars().next() {
            out.extend(c.to_uppercase());
            out.push('.');
        }
    }
    out
}
