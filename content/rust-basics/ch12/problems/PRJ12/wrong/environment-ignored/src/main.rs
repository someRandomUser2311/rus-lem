use std::env;
use std::fs;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::build(&args, false) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("minigrep: {msg}");
            process::exit(1);
        }
    };
    let contents = match fs::read_to_string(&config.file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("minigrep: {}: {e}", config.file_path);
            process::exit(1);
        }
    };
    print!("{}", run(&config, &contents));
}
