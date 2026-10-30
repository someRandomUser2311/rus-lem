use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Type a whole number:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: u32 = number.trim().parse().unwrap();
    match number.cmp(&50) {
        Ordering::Less => println!("{number} is below 50."),
        Ordering::Greater => println!("{number} is above 50."),
        Ordering::Equal => println!("{number} is exactly 50!"),
    }
}
