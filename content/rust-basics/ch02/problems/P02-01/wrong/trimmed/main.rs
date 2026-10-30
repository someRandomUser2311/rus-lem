use std::io;
fn main() {
    println!("Say something:");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    println!("You said: {}", line.trim());
}
