pub const LOAN_DAYS: u32 = 14;
pub const PREMIUM_LOAN_DAYS: u32 = 28;
pub const FEE_CAP_CENTS: u32 = 2_000;
pub fn due_day(borrow_day: u32, premium: bool) -> u32 {
    let duration = if premium { PREMIUM_LOAN_DAYS } else { LOAN_DAYS };
    borrow_day + duration
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
    format!("${:.2}", cents as f64 / 100.0)
}
use std::io;
fn main() {
    println!("==============================");
    println!("  Riverside Community Library");
    println!("==============================");
    println!("Open daily, 9:00 to 17:00");
    loop {
        println!("1) List books");
        println!("2) Borrow a book");
        println!("3) Return a book");
        println!("4) Quit");
        println!("Choose an option:");
        let mut choice = String::new();
        let bytes = io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match bytes {
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => {}
        }
        let choice: u32 = match choice.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a number from 1 to 4.");
                continue;
            }
        };
        match choice {
            1 => println!("You chose: List books"),
            2 => println!("You chose: Borrow a book"),
            3 => println!("You chose: Return a book"),
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Please type a number from 1 to 4."),
        }
    }
}
