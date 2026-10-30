use std::io;
fn main() {
    let mut initial_text = String::new();
    io::stdin()
        .read_line(&mut initial_text)
        .expect("Failed to read line");
    match initial_text.trim().parse::<u32>() {
        Ok(initial) => {
            let mut counter = initial;
            println!("Before: {counter}");
            counter = 20;
            println!("After: {counter}");
        }
        Err(_) => println!("Input error"),
    }
}
