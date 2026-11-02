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
        assert_eq!(fee(8), 225, "day 8 is the first at 50");
        assert_eq!(fee(43), 1975, "43 days is the last fee under the cap");
    }

    #[test]
    fn fee_is_capped_at_2000() {
        assert_eq!(fee(44), 2000, "44 days would be 2025");
        assert_eq!(fee(u32::MAX), 2000, "the longest loan is capped too");
    }

}
