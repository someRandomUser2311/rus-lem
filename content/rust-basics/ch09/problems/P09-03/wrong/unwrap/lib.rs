pub fn count(text: &str) -> Result<u32, String> {
    Ok(text.trim().parse::<u32>().unwrap())
}
