use std::io;
fn main() {
    let mut width_text = String::new();
    io::stdin()
        .read_line(&mut width_text)
        .expect("Failed to read line");
    match width_text.trim().parse::<u32>() {
        Ok(width) => {
            let mut height_text = String::new();
            io::stdin()
                .read_line(&mut height_text)
                .expect("Failed to read line");
            match height_text.trim().parse::<u32>() {
                Ok(height) => {
                    let size = (width, height, 'R');
                    println!("{} x {} ({})", size.0, size.1, size.2);
                }
                Err(_) => println!("Input error"),
            }
        }
        Err(_) => println!("Input error"),
    }
}
