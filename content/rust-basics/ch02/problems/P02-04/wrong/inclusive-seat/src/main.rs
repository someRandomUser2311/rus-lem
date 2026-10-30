use rand::prelude::*;
fn main() {
    let table: u32 = rand::rng().random_range(1..=20);
    let seat: u32 = rand::rng().random_range(1..=4);
    println!("Your table: {table}");
    println!("Your seat: {seat}");
}
