pub fn calendar(year: u32, month: u32) -> (bool, u32) {
    let leap = {
        let by_four = year % 4 == 0;
        (by_four && year % 100 != 0) || year % 400 == 0
    };
    let mut days_in_month = 0;
    if month == 2 {
        days_in_month = 28;
        if leap {
            days_in_month = 29;
        }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        days_in_month = 30;
    } else if month >= 1 && month <= 12 {
        days_in_month = 31;
    }
    (leap, days_in_month)
}
