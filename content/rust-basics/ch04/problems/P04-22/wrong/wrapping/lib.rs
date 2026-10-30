pub fn adjust(scores: &mut [u32], bonus: u32) -> () {
    for i in 0..scores.len() {
        scores[i] = scores[i].wrapping_add(bonus);
    }
}
