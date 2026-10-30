use std::cmp::Ordering;
use std::io;
use rand::prelude::*;
fn main() {
    println!("Rock, paper, scissors!");
    let mut last: u32 = 0;
    let computer: u32 = rand::rng().random_range(1..=3);
    loop {
        println!("Your move (rock, paper, scissors, again, quit):");
        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line).expect("Failed to read line");
        match bytes {
            0 => {
                println!("Thanks for playing!");
                break;
            }
            _ => {}
        }
        let player: u32 = match line.trim() {
            "rock" => 1,
            "r" => 1,
            "paper" => 2,
            "p" => 2,
            "scissors" => 3,
            "s" => 3,
            "again" => last,
            "quit" => {
                println!("Thanks for playing!");
                break;
            }
            _ => {
                println!("I don't know that move.");
                continue;
            }
        };
        match player {
            0 => {
                println!("No previous move yet.");
                continue;
            }
            _ => {}
        }
        last = player;
        let player_name = match player {
            1 => "rock",
            2 => "paper",
            _ => "scissors",
        };
        let computer_name = match computer {
            1 => "rock",
            2 => "paper",
            _ => "scissors",
        };
        println!("You chose {player_name}. Computer chose {computer_name}.");
        match player.cmp(&computer) {
            Ordering::Equal => println!("It's a tie!"),
            _ => match player {
                1 => match computer {
                    3 => println!("You win!"),
                    _ => println!("You lose!"),
                },
                2 => match computer {
                    1 => println!("You win!"),
                    _ => println!("You lose!"),
                },
                _ => match computer {
                    2 => println!("You win!"),
                    _ => println!("You lose!"),
                },
            },
        }
    }
}
