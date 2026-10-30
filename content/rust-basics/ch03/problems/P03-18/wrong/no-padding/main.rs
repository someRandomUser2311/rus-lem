use std::io;
pub fn countdown(n: u32) {
    for value in (1..=n).rev() {
        println!("{value}");
    }
    println!("Go!");
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    match line.trim().parse::<u32>() {
        Ok(n) => countdown(n),
        Err(_) => println!("Input error"),
    }
}
