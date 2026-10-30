use std::io;
fn main() {
    let mut attempt = String::new();
    loop {
        println!("Password:");
        let bytes = io::stdin().read_line(&mut attempt).expect("Failed to read line");
        match bytes {
            0 => {
                println!("No more input. Locked.");
                break;
            }
            _ => {}
        }
        match attempt.trim() {
            "rustacean" => {
                println!("Welcome!");
                break;
            }
            "hint" => println!("Hint: it's what Rust programmers call themselves."),
            "" => println!("Please type something."),
            _ => println!("Wrong password, try again."),
        }
    }
}
