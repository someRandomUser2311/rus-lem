use std::io;
fn main() {
    println!("Say something:");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .unwrap();
    print!("You said: {line}");
}
