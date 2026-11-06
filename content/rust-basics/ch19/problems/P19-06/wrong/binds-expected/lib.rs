pub const LIMIT: i32 = 100;

pub fn check(value: Option<i32>, expected: i32) -> String {
    match value {
        None => String::from("missing"),
        Some(expected) => String::from("match"),
        Some(v) => format!("other {v}"),
    }
}

pub fn at_limit(v: i32) -> bool {
    match v {
        LIMIT => true,
        _ => false,
    }
}

pub fn positions(values: &[i32], target: i32) -> Vec<usize> {
    let mut out = Vec::new();
    for (i, &v) in values.iter().enumerate() {
        match v {
            x if x == target => out.push(i),
            _ => {}
        }
    }
    out
}
