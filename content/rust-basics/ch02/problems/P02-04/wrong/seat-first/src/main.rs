use rand::prelude::*;
fn main() {
    let seat: u32 = rand::rng().random_range(1..5);
    let table: u32 = rand::rng().random_range(1..=20);
    println!("Your table: {table}");
    println!("Your seat: {seat}");
}
