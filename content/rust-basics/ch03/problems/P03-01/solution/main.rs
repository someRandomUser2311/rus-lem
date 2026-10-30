use std::io;
fn main() {
    let mut new_score_text = String::new();
    io::stdin()
        .read_line(&mut new_score_text)
        .expect("Failed to read line");
    match new_score_text.trim().parse::<u32>() {
        Ok(new_score) => {
            let mut score = 0;
            println!("Before: {score}");
            score = new_score;
            println!("After: {score}");
        }
        Err(_) => println!("Input error"),
    }
}
