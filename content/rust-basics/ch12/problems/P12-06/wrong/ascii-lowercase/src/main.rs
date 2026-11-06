use std::env;
use std::fs;
use std::process;

use minigrep::{ignore_case_from, search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Problem parsing arguments: expected <query> <file>");
        process::exit(1);
    }
    let query = &args[1];
    let file_path = &args[2];
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Application error: {file_path}: {e}");
            process::exit(1);
        }
    };
    let ignore_case = ignore_case_from(env::var("IGNORE_CASE").ok().as_deref());
    let results = if ignore_case {
        search_case_insensitive(query, &contents)
    } else {
        search(query, &contents)
    };
    for line in results {
        println!("{line}");
    }
}
