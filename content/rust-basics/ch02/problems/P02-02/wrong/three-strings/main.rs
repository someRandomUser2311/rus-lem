use std::io;
fn main() {
    println!("Type three guest names, one per line:");
    let mut first = String::new();
    let mut second = String::new();
    let mut third = String::new();
    io::stdin().read_line(&mut first).expect("Failed to read line");
    io::stdin().read_line(&mut second).expect("Failed to read line");
    io::stdin().read_line(&mut third).expect("Failed to read line");
    println!("Guest list:");
    print!("{first}{second}{third}");
}
