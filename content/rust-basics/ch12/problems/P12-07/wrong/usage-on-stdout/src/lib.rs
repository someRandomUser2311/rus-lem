use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub count: usize,
    pub file_path: String,
}

fn parse_count(text: &str) -> Option<usize> {
    if text.is_empty() {
        return None;
    }
    for c in text.chars() {
        if !c.is_ascii_digit() {
            return None;
        }
    }
    text.parse::<usize>().ok()
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("missing file path");
        }
        if args[1] == "-n" {
            if args.len() < 3 {
                return Err("missing line count");
            }
            let Some(count) = parse_count(&args[2]) else {
                return Err("line count must be a whole number");
            };
            if args.len() < 4 {
                return Err("missing file path");
            }
            if args.len() > 4 {
                return Err("too many arguments");
            }
            return Ok(Config {
                count,
                file_path: args[3].clone(),
            });
        }
        if args[1].starts_with('-') && args[1].len() > 1 {
            return Err("unknown option");
        }
        if args.len() > 2 {
            return Err("too many arguments");
        }
        Ok(Config {
            count: 10,
            file_path: args[1].clone(),
        })
    }
}

pub fn head_text(text: &str, count: usize) -> String {
    let mut out = String::new();
    let mut taken = 0;
    for line in text.lines() {
        if taken == count {
            break;
        }
        out.push_str(line);
        out.push('\n');
        taken += 1;
    }
    out
}

pub fn run(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    Ok(head_text(&contents, config.count))
}
