use rand::prelude::*;
fn main() {
    let roll: u32 = rand::rng().random_range(1..6);
    println!("You rolled a {roll}!");
}
