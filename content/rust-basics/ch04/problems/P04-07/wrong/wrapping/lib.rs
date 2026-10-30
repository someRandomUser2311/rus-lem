pub fn with_bonus(scores: [u32; 5], bonus: u32) -> ([u32; 5], [u32; 5]) {
    let mut boosted = scores;
    for i in 0..5 {
        boosted[i] = boosted[i].wrapping_add(bonus);
    }
    (scores, boosted)
}
