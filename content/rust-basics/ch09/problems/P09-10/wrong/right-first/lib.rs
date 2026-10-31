pub fn optional_add(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    a?.checked_add(b?)
}

pub fn add_counts(left: &str, right: &str) -> Result<u32, String> {
    let b = right.trim().parse::<u32>().ok().ok_or(String::from("right"))?;
    let a = left.trim().parse::<u32>().ok().ok_or(String::from("left"))?;
    optional_add(Some(a), Some(b)).ok_or(String::from("overflow"))
}
