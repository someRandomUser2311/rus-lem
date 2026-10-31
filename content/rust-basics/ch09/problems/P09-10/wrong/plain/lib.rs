pub fn optional_add(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    Some(a? + b?)
}

pub fn add_counts(left: &str, right: &str) -> Result<u32, String> {
    let a = left.trim().parse::<u32>().ok().ok_or(String::from("left"))?;
    let b = right.trim().parse::<u32>().ok().ok_or(String::from("right"))?;
    optional_add(Some(a), Some(b)).ok_or(String::from("overflow"))
}
