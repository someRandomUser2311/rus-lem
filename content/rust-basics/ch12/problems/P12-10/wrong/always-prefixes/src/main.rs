use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    let config = match Config::build(&args, ignore_case) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("multigrep: {msg}");
            process::exit(2);
        }
    };
    let outcome = run(&config);
    print!("{}", outcome.output);
    for e in &outcome.errors {
        eprintln!("multigrep: {e}");
    }
    process::exit(outcome.exit_code);
}
