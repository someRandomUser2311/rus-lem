pub async fn increment(n: u32) -> u32 {
    n.wrapping_add(1)
}
