const CENTS_PER_DOLLAR: u64 = 100;
use std::io;
fn main() {
    let mut quantity_text = String::new();
    io::stdin()
        .read_line(&mut quantity_text)
        .expect("Failed to read line");
    match quantity_text.trim().parse::<u32>() {
        Ok(quantity) => {
            let mut price_text = String::new();
            io::stdin()
                .read_line(&mut price_text)
                .expect("Failed to read line");
            match price_text.trim().parse::<u32>() {
                Ok(price) => {
                    let total = u64::from(quantity) * u64::from(price);
                    let dollars = total / CENTS_PER_DOLLAR;
                    let cents = total % CENTS_PER_DOLLAR;
                    println!("Total: ${dollars}.{cents:02}");
                }
                Err(_) => println!("Input error"),
            }
        }
        Err(_) => println!("Input error"),
    }
}
