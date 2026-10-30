use std::io;
fn main() {
    println!("Type three guest names, one per line:");
    let mut guests = String::new();
    io::stdin()
        .read_line(&mut guests)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut guests)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut guests)
        .expect("Failed to read line");
    println!("Guest list:");
    print!("{guests}");
}
