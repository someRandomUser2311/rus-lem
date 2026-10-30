use std::io;
fn main() {
    loop {
        println!("Password:");
        let mut attempt = String::new();
        let bytes = io::stdin().read_line(&mut attempt).expect("Failed to read line");
        if bytes == 0 {
            println!("No more input. Locked.");
            break;
        }
        let attempt = attempt.trim();
        if attempt == "rustacean" {
            println!("Welcome!");
            break;
        } else if attempt == "hint" {
            println!("Hint: it's what Rust programmers call themselves.");
        } else if attempt == "" {
            println!("Please type something.");
        } else {
            println!("Wrong password, try again.");
        }
    }
}
