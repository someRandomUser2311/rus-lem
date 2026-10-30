use std::io;
fn main() {
    let mut a_text = String::new();
    io::stdin()
        .read_line(&mut a_text)
        .expect("Failed to read line");
    match a_text.trim().parse::<u8>() {
        Ok(a) => {
            let mut b_text = String::new();
            io::stdin()
                .read_line(&mut b_text)
                .expect("Failed to read line");
            match b_text.trim().parse::<u8>() {
                Ok(b) => {
                    let wrapped = a + b;
                    let saturated = a.saturating_add(b);
                    let pair = a.overflowing_add(b);
                    println!("{wrapped} {saturated}");
                    println!("{pair:?}");
                }
                Err(_) => println!("Input error"),
            }
        }
        Err(_) => println!("Input error"),
    }
}
