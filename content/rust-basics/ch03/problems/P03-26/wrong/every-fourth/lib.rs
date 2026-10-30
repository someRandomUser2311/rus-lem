pub fn calendar(year: u32, month: u32) -> (bool, u32) {
    let leap = {
        let by_four = year % 4 == 0;
        by_four
    };
    let days_in_month = if month == 2 {
        if leap { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else if month >= 1 && month <= 12 {
        31
    } else {
        0
    };
    (leap, days_in_month)
}
