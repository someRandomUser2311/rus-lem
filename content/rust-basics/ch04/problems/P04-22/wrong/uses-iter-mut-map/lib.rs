pub fn adjust(scores: &mut [u32], bonus: u32) -> () {
    scores.iter_mut().map(|s| *s = s.saturating_add(bonus)).count();
}
