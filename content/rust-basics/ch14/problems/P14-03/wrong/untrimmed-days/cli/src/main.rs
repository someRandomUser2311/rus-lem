use std::env;
use std::process;

use course_core::{format_cents, late_fee};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: course-cli <days>");
        process::exit(2);
    }
    let days: u32 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("course-cli: invalid days: {}", args[1]);
            process::exit(1);
        }
    };
    println!("{}", format_cents(late_fee(days)));
}
