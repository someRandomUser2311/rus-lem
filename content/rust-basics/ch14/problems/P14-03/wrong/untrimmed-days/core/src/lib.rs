//! Late fees for the course library.

/// The fee for returning a book `days` late: 25 cents a day, at most $10.00.
///
/// # Examples
///
/// ```
/// use course_core::late_fee;
///
/// assert_eq!(late_fee(3), 75);
/// assert_eq!(late_fee(1_000), 1_000);
/// ```
pub fn late_fee(days: u32) -> u32 {
    days.saturating_mul(25).min(1_000)
}

/// Writes `cents` as dollars and cents.
///
/// # Examples
///
/// ```
/// use course_core::format_cents;
///
/// assert_eq!(format_cents(125), "$1.25");
/// assert_eq!(format_cents(5), "$0.05");
/// ```
pub fn format_cents(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}
