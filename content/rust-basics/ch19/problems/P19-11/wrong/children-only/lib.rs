pub fn price(age: u32, student: bool) -> u32 {
    match age {
        0..=2 => 0,
        3..=12 => 5,
        13..=17 | 65.. => 7,
        _ if student => 7,
        _ => 10,
    }
}

pub fn label(age: u32) -> String {
    match age {
        n @ 0..=12 if n % 2 == 0 => format!("even special {n}"),
        n @ (0..=12 | 65..) => format!("special {n}"),
        n => format!("standard {n}"),
    }
}
