const SECONDS_PER_HOUR: u32 = 60 * 60;
use std::io;
fn main() {
    let mut hours_text = String::new();
    io::stdin()
        .read_line(&mut hours_text)
        .expect("Failed to read line");
    match hours_text.trim().parse::<u32>() {
        Ok(hours) => {
            let seconds = hours * SECONDS_PER_HOUR;
            println!("{seconds}");
        }
        Err(_) => println!("Input error"),
    }
}
