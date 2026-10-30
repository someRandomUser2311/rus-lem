use std::io;
fn main() {
    let mut a_text = String::new();
    io::stdin()
        .read_line(&mut a_text)
        .expect("Failed to read line");
    match a_text.trim().parse::<i32>() {
        Ok(a) => {
            let mut b_text = String::new();
            io::stdin()
                .read_line(&mut b_text)
                .expect("Failed to read line");
            match b_text.trim().parse::<i32>() {
                Ok(b) => {
                    if b == 0 {
                        println!("Division by zero");
                    } else {
                        let quotient = a / b;
                        let remainder = a % b;
                        println!("{quotient} {remainder}");
                    }
                }
                Err(_) => println!("Input error"),
            }
        }
        Err(_) => println!("Input error"),
    }
}
