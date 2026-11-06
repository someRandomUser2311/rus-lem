use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match amounts_core::total(&refs) {
        Ok(sum) => println!("{sum}"),
        Err(e) => {
            eprintln!("amounts: {e}");
            process::exit(1);
        }
    }
}
