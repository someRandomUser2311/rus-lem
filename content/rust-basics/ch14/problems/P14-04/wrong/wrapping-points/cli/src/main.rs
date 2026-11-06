use std::env;
use std::process;

fn number(text: &str) -> i32 {
    match text.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("score-cli: invalid number: {text}");
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: score-cli <label> <points> <delta>");
        process::exit(2);
    }
    let points = number(&args[2]);
    let delta = number(&args[3]);
    let score = score_models::Score {
        label: args[1].trim().to_string(),
        points,
    };
    match score_core::revise(score, delta) {
        Some(s) => println!("{}: {}", s.label, s.points),
        None => {
            eprintln!("score-cli: overflow");
            process::exit(1);
        }
    }
}
