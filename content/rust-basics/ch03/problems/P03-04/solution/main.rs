use std::io;
fn main() {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    match value.trim().parse::<u32>() {
        Ok(number) => {
            let value = number;
            println!("{value}");
        }
        Err(_) => println!("Input error"),
    }
}
