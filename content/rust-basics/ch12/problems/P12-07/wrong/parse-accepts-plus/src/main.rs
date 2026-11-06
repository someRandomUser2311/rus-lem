use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::build(&args) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("head: {msg}");
            eprintln!("usage: head [-n COUNT] <file>");
            process::exit(2);
        }
    };
    match run(&config) {
        Ok(out) => print!("{out}"),
        Err(e) => {
            eprintln!("head: {}: {e}", config.file_path);
            process::exit(1);
        }
    }
}
