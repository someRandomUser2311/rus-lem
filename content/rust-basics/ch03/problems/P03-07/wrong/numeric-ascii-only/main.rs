use std::io;
fn main() {
    let mut c_text = String::new();
    io::stdin()
        .read_line(&mut c_text)
        .expect("Failed to read line");
    match c_text.trim().parse::<char>() {
        Ok(c) => {
            let alphabetic = c.is_alphabetic();
            let numeric = c.is_ascii_digit();
            let digit = c.is_ascii_digit();
            let upper = c.is_uppercase();
            let to_upper = c.to_ascii_uppercase();
            let to_lower = c.to_ascii_lowercase();
            println!("{alphabetic} {numeric} {digit} {upper} {to_upper} {to_lower}");
        }
        Err(_) => println!("Input error"),
    }
}
