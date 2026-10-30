use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Type a whole number:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let text = number.trim().to_string();
    let number: u32 = number.trim().parse().expect("Please type a number!");
    match number.cmp(&50) {
        Ordering::Less => println!("{text} is below 50."),
        Ordering::Greater => println!("{text} is above 50."),
        Ordering::Equal => println!("{text} is exactly 50!"),
    }
}
