use std::io;
fn main() {
    println!("Type a whole number:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: u32 = number.trim().parse().expect("Please type a number!");
    if number < 50 {
        println!("{number} is below 50.");
    } else if number > 50 {
        println!("{number} is above 50.");
    } else {
        println!("{number} is exactly 50!");
    }
}
