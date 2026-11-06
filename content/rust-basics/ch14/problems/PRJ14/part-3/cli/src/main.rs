use std::env;
use std::fs;
use std::process;

use analysis_core::{Sale, paid_total, report};

fn sale(line: &str) -> Option<Sale> {
    let fields: Vec<&str> = line.split(',').map(|f| f.trim()).collect();
    if fields.len() != 4 {
        return None;
    }
    let paid = match fields[3] {
        "yes" => true,
        "no" => false,
        _ => return None,
    };
    Some(Sale {
        product: fields[0].to_string(),
        quantity: fields[1].parse().ok()?,
        unit_cents: fields[2].parse().ok()?,
        paid,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: analysis-cli <sales.csv>");
        process::exit(2);
    }
    let path = &args[1];
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("analysis-cli: {path}: {e}");
            process::exit(1);
        }
    };
    let mut sales = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match sale(line) {
            Some(s) => sales.push(s),
            None => {
                eprintln!(
                    "analysis-cli: line {}: expected product,quantity,unit_cents,yes|no",
                    i + 1
                );
                process::exit(1);
            }
        }
    }
    let total = paid_total(&sales);
    print!("{}", report(&sales));
    println!("Total paid: ${}.{:02}", total / 100, total % 100);
}
