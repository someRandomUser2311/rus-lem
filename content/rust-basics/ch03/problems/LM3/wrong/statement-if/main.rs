pub const LOAN_DAYS: u32 = 14;
pub const PREMIUM_LOAN_DAYS: u32 = 28;
pub const FEE_CAP_CENTS: u32 = 2_000;
pub fn due_day(borrow_day: u32, premium: bool) -> u32 {
    if premium {
        return borrow_day + PREMIUM_LOAN_DAYS;
    }
    borrow_day + LOAN_DAYS
}
pub fn days_late(due_day: u32, return_day: u32) -> u32 {
    // Avoid subtraction when the return is early.
    0
}
pub fn late_fee_cents(days_late: u32) -> u32 {
    // Apply the tiers without overflowing before applying the cap.
    0
}
pub fn format_cents(cents: u32) -> String {
    // Split dollars and cents using integer arithmetic.
    format!("$0.00")
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
