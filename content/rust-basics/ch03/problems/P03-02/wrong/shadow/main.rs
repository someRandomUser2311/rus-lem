use std::io;
fn main() {
    let mut initial_text = String::new();
    io::stdin()
        .read_line(&mut initial_text)
        .expect("Failed to read line");
    match initial_text.trim().parse::<u32>() {
        Ok(initial) => {
            let counter = initial;
            println!("Before: {counter}");
            let counter = 20;
            println!("After: {counter}");
        }
        Err(_) => println!("Input error"),
    }
}
