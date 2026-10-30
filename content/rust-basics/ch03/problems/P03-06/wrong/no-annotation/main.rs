use std::io;
fn main() {
    let mut c_text = String::new();
    io::stdin()
        .read_line(&mut c_text)
        .expect("Failed to read line");
    match c_text.trim().parse::<i32>() {
        Ok(c) => {
            let f = c as f64;
            let fahrenheit = f * 9.0 / 5.0 + 32.0;
            println!("{fahrenheit:.2}");
        }
        Err(_) => println!("Input error"),
    }
}
