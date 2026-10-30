use rand::prelude::*;
fn main() {
    let roll: u32 = rand::rng().random_range(7..=12);
    println!("You rolled a {roll}!");
}
