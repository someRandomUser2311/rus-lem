use std::env;
use std::process;

use catalog_core::{Record, first_named, render};

fn record(text: &str) -> Option<Record> {
    let (id, name) = text.split_once(':')?;
    let id = id.parse().ok()?;
    Some(Record {
        id,
        name: name.to_string(),
    })
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some((name, rest)) = args.split_first() else {
        eprintln!("usage: catalog-cli <name> [id:name]...");
        process::exit(2);
    };
    let mut records = Vec::new();
    for text in rest {
        match record(text) {
            Some(r) => records.push(r),
            None => {
                eprintln!("catalog-cli: bad record: {text}");
                process::exit(1);
            }
        }
    }
    print!("{}", render(&records));
    match first_named(&records, name) {
        Some(r) => println!("found {}", r.id),
        None => println!("not found"),
    }
}
