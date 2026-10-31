pub const LOAN_DAYS: u32 = 14;
pub const PREMIUM_LOAN_DAYS: u32 = 28;
pub const FEE_CAP_CENTS: u32 = 2_000;

pub fn due_day(borrow_day: u32, premium: bool) -> u32 {
    borrow_day
        + if premium {
            PREMIUM_LOAN_DAYS
        } else {
            LOAN_DAYS
        }
}
pub fn days_late(due_day: u32, return_day: u32) -> u32 {
    if return_day > due_day {
        return_day - due_day
    } else {
        0
    }
}
pub fn late_fee_cents(days_late: u32) -> u32 {
    if days_late >= 44 {
        FEE_CAP_CENTS
    } else if days_late <= 7 {
        days_late * 25
    } else {
        175 + (days_late - 7) * 50
    }
}
pub fn format_cents(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}
