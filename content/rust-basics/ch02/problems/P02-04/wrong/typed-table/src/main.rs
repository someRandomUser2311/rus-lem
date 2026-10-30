use rand::prelude::*;
fn main() {
    let table: u32 = rand::rng().random_range(1..=20);
    let seat: u32 = rand::rng().random_range(1..5);
    println!("Your table: 13");
    println!("Your seat: {seat}");
}
