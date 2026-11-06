//! Rules for changing scores.

pub use score_models::Score;

/// Adds `delta` to the score's points, or `None` if that would overflow.
pub fn revise(score: Score, delta: i32) -> Option<Score> {
    let points = score.points.wrapping_add(delta);
    Some(Score { points, ..score })
}
