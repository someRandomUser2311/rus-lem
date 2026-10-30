use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number: u32 = 42;
    println!("Please input your guess.");
    let mut guess = String::new();
    let bytes = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    match bytes {
        0 => {
            println!("Goodbye!");
            return;
        }
        _ => {}
    }
    match guess.trim().parse::<u32>() {
        Ok(guess) => println!("You guessed: {guess}"),
        Err(_) => println!("Please type a number!"),
    }
}
