//! Small arithmetic helpers that never overflow.

/// Adds two numbers, stopping at `u32::MAX` instead of overflowing.
///
/// # Examples
///
/// ```
/// use doclab::capped_add;
///
/// assert_eq!(capped_add(2, 3), 6);
/// assert_eq!(capped_add(u32::MAX, 1), u32::MAX);
/// ```
///
/// # Panics
///
/// Never: every pair of `u32`s has a result.
pub fn capped_add(a: u32, b: u32) -> u32 {
    a.saturating_add(b)
}
