use std::io;
fn main() {
    loop {
        println!("Password:");
        let mut attempt = String::new();
        let bytes = io::stdin().read_line(&mut attempt).expect("Failed to read line");
        match bytes {
            0 => {
                println!("No more input. Locked.");
                break;
            }
            _ => {}
        }
        match attempt.as_str() {
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
