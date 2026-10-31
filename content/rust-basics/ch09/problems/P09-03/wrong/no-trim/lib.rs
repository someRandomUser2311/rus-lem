pub fn count(text: &str) -> Result<u32, String> {
    match text.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("invalid count: {text}")),
    }
}
