use std::io;
fn main() {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    match value.trim().parse::<u32>() {
        Ok(number) => {
            println!("{number}");
        }
        Err(_) => println!("Input error"),
    }
}
