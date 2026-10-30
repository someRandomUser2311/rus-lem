use std::io;
fn main() {
    loop {
        println!("==============================");
        println!("  Riverside Community Library");
        println!("==============================");
        println!("Open daily, 9:00 to 17:00");
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
            Ok(n) => n,
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
