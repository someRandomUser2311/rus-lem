pub fn add_one(x: i32) -> i32 {
    x.saturating_add(2)
}

pub fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

pub fn to_strings(values: &[i32]) -> Vec<String> {
    values.iter().map(ToString::to_string).collect()
}

pub fn wrap_all(values: Vec<u32>) -> Vec<Option<u32>> {
    values.into_iter().map(Some).collect()
}
