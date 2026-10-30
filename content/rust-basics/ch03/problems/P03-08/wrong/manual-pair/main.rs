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
                    let wrapped = a.wrapping_add(b);
                    let saturated = a.saturating_add(b);
                    let pair = (a.wrapping_add(b), a.checked_add(b).is_none());
                    println!("{wrapped} {saturated}");
                    println!("{pair:?}");
                }
                Err(_) => println!("Input error"),
            }
        }
        Err(_) => println!("Input error"),
    }
}
