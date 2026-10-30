pub fn with_bonus(scores: [u32; 5], bonus: u32) -> ([u32; 5], [u32; 5]) {
    let mut boosted = scores.clone();
    for i in 0..5 {
        boosted[i] = boosted[i].saturating_add(bonus);
    }
    (scores, boosted)
}
