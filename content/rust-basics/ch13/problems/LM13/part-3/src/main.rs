use std::env;
use std::fs;
use std::process;

use library::cli::{Config, run};

fn main() {
    let config = match Config::build(env::args(), env::var("SORT").ok()) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("library: {msg}");
            process::exit(2);
        }
    };
    let catalog = match fs::read_to_string(&config.catalog_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("library: {}: {e}", config.catalog_path);
            process::exit(1);
        }
    };
    match run(&config, &catalog) {
        Ok(out) => print!("{out}"),
        Err(msg) => {
            eprintln!("library: {}: {msg}", config.catalog_path);
            process::exit(1);
        }
    }
}
