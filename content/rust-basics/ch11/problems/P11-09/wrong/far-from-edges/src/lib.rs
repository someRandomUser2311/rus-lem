#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_week_costs_25_a_day() {
        assert_eq!(fee(0), 0, "zero days cost nothing");
        assert_eq!(fee(1), 25, "one day in the first week");
        assert_eq!(fee(7), 175, "day 7 is the last at 25");
    }

    #[test]
    fn later_days_cost_50() {
        assert_eq!(fee(10), 325, "day 10 is at 50");
        assert_eq!(fee(20), 825, "20 days is under the cap");
    }

    #[test]
    fn fee_is_capped_at_2000() {
        assert_eq!(fee(50), 2000, "50 days is over the cap");
        assert_eq!(fee(u32::MAX), 2000, "the longest loan is capped too");
    }

    #[test]
    fn raw_fee_is_exact_for_long_loans() {
        assert_eq!(raw_fee(50), 2325, "the helper doesn't cap");
        assert_eq!(raw_fee(u32::MAX), 214_748_364_575, "the helper works in u64 without overflow");
    }
}
