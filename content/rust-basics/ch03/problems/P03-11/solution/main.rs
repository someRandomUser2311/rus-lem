use std::io;
fn main() {
    let mut index_text = String::new();
    io::stdin()
        .read_line(&mut index_text)
        .expect("Failed to read line");
    match index_text.trim().parse::<usize>() {
        Ok(index) => {
            let days = [31, 28, 31, 30, 31];
            let i: usize = index;
            println!("{}", days[i]);
        }
        Err(_) => println!("Input error"),
    }
}
