use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Type numbers, one per line. Type done to finish.");
    let mut largest: u32 = 0;
    loop {
        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line).expect("Failed to read line");
        match bytes {
            0 => break,
            _ => {}
        }
        match line.trim() {
            "done" => break,
            "" => continue,
            _ => {}
        }
        let line: u32 = match line.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Skipping: {}", line.trim());
                continue;
            }
        };
        match line.cmp(&largest) {
            Ordering::Greater => {
                largest = line;
                println!("New largest: {largest}");
            }
            _ => {}
        }
    }
    println!("Largest: {largest}");
}
