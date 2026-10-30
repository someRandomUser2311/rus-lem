pub fn adjust(scores: &mut [u32], bonus: u32) -> () {
    for i in 0..scores.len().saturating_sub(1) {
        scores[i] = scores[i].saturating_add(bonus);
    }
}
