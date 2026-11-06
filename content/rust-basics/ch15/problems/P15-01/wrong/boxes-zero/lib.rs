pub fn boxed_score(n: u32) -> Box<u32> {
    Box::new(n / 2 * 2)
}
