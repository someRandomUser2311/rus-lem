use std::io;
pub fn countdown(mut n: u32) {
    while n > 0 {
        println!("{n:03}");
        n -= 1;
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
