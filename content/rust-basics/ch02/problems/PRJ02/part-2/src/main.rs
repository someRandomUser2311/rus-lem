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
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            println!("Goodbye!");
            return;
        }
    };
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return;
        }
    }
    println!("Goodbye!");
}
