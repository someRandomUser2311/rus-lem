use std::io;
fn main() {
    let mut c_text = String::new();
    io::stdin()
        .read_line(&mut c_text)
        .expect("Failed to read line");
    match c_text.trim().parse::<i32>() {
        Ok(c) => {
            let fahrenheit: f64 = ((c * 9) / 5 + 32) as f64;
            println!("{fahrenheit:.2}");
        }
        Err(_) => println!("Input error"),
    }
}
