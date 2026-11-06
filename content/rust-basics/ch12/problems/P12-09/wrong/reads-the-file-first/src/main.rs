use std::env;
use std::fs;
use std::process;

use minigrep::{parse_spec, select};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("lines: expected <spec> <file>");
        eprintln!("usage: lines <spec> <file>");
        process::exit(2);
    }
    let contents = match fs::read_to_string(&args[2]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("lines: {}: {e}", args[2]);
            process::exit(1);
        }
    };
    let ranges = match parse_spec(&args[1]) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("lines: {msg}");
            eprintln!("usage: lines <spec> <file>");
            process::exit(2);
        }
    };
    print!("{}", select(&ranges, &contents));
}
