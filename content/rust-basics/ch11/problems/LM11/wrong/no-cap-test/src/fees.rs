#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_and_premium_loans() {
        assert_eq!(due_day(10, false), 24, "a standard loan lasts 14 days");
        assert_eq!(due_day(10, true), 38, "a premium loan lasts 28 days");
    }

    #[test]
    fn early_and_on_time_returns_are_not_late() {
        assert_eq!(days_late(24, 20), 0, "returned four days early");
        assert_eq!(days_late(24, 24), 0, "returned on the due day");
        assert_eq!(days_late(24, 25), 1, "one day late");
    }

    #[test]
    fn fees_follow_the_two_rates_and_the_cap() {
        assert_eq!(late_fee_cents(7), 175, "the last day at 25 cents");
        assert_eq!(late_fee_cents(8), 225, "the first day at 50 cents");
        assert_eq!(late_fee_cents(43), 1975, "the last fee under the cap");
    }
}
