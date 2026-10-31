use std::error::Error;
use std::io;

fn line_total(line: &str, number: usize) -> Result<u64, String> {
    let mut total: u64 = 0;
    for token in line.split_whitespace() {
        let amount: u32 = match token.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        total += amount as u64;
    }
    Ok(total)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut totals = Vec::new();
    let mut number = 0;
    for line in io::stdin().lines() {
        let line = line?;
        number += 1;
        if line.trim().is_empty() {
            continue;
        }
        let total = line_total(&line, number)?;
        totals.push((number, total));
    }
    let mut grand: u64 = 0;
    for (number, total) in totals {
        println!("line {number}: {total}");
        grand += total;
    }
    println!("total: {grand}");
    Ok(())
}
