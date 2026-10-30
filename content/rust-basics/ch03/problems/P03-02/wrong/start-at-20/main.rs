use std::io;
fn main() {
    let mut initial_text = String::new();
    io::stdin()
        .read_line(&mut initial_text)
        .expect("Failed to read line");
    match initial_text.trim().parse::<u32>() {
        Ok(initial) => {
            let mut counter = initial;
            counter = 20;
            println!("Before: {counter}");
            println!("After: {counter}");
            let _ = initial;
        }
        Err(_) => println!("Input error"),
    }
}
