pub fn optional_add(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    a?.checked_add(b?)
}

pub fn add_counts(left: &str, right: &str) -> Result<u32, String> {
    let a = left.trim().parse::<u64>().ok().ok_or(String::from("left"))?;
    let b = right.trim().parse::<u64>().ok().ok_or(String::from("right"))?;
    if a + b > 4294967295 {
        return Err(String::from("overflow"));
    }
    optional_add(Some(a as u32), Some(b as u32)).ok_or(String::from("x"))
}
