#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub line_numbers: bool,
    pub count: bool,
    pub invert: bool,
}

const USAGE: &str = "usage: minigrep [-i|-s] [-n] [-c] [-v] <query> <file>";

impl Config {
    pub fn build(args: &[String], ignore_case_env: bool) -> Result<Config, String> {
        let mut ignore_case = ignore_case_env;
        let mut line_numbers = false;
        let mut count = false;
        let mut invert = false;
        let mut i = 1;
        while i < args.len() {
            let arg = args[i].as_str();
            if arg == "--" {
                i += 1;
                break;
            }
            if !arg.starts_with('-') || arg.len() == 1 {
                break;
            }
            match arg {
                "-i" => ignore_case = true,
                "-s" => ignore_case = false,
                "-n" => line_numbers = true,
                "-c" => count = true,
                "-v" => invert = true,
                _ => return Err(format!("unknown option: {arg}")),
            }
            i += 1;
        }
        if args.len() != i + 2 {
            return Err(String::from(USAGE));
        }
        Ok(Config {
            query: args[i].clone(),
            file_path: args[i + 1].clone(),
            ignore_case,
            line_numbers,
            count,
            invert,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: &Config, contents: &str) -> String {
    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query.clone()
    };
    let mut out = String::new();
    let mut selected = 0;
    let mut number = 0;
    for line in contents.lines() {
        number += 1;
        let found = if config.ignore_case {
            line.to_lowercase().contains(&query)
        } else {
            line.contains(&query)
        };
        if found != config.invert {
            selected += 1;
            if !config.count || config.line_numbers {
                if config.line_numbers {
                    out.push_str(&format!("{number}: "));
                }
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    if config.count && !config.line_numbers {
        format!("{selected}\n")
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["safe, fast, productive."], search("duct", contents));
    }

    #[test]
    fn case_insensitive() {
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive("rUsT", contents));
    }

    #[test]
    fn empty_inputs() {
        assert_eq!(Vec::<&str>::new(), search("x", ""));
        assert_eq!(vec!["a", "b"], search("", "a\nb"));
    }
}
