pub fn shift(scores: &mut [i32], delta: i32) {
    for s in scores {
        *s = s.wrapping_add(delta);
    }
}
