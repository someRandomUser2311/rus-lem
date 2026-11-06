use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub paths: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String], ignore_case_env: bool) -> Result<Config, &'static str> {
        let mut i = 1;
        let mut flag = false;
        if args.len() > 1 && args[1] == "-i" {
            flag = true;
            i = 2;
        }
        if args.len() < i + 2 {
            return Err("usage: multigrep [-i] <query> <file>...");
        }
        Ok(Config {
            query: args[i].clone(),
            paths: args[i + 1..].to_vec(),
            ignore_case: flag || ignore_case_env,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Outcome {
    pub output: String,
    pub errors: Vec<String>,
    pub exit_code: i32,
}

pub fn matching_lines<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    let mut results = Vec::new();
    for line in contents.lines() {
        let found = if ignore_case {
            line.to_lowercase().contains(&query)
        } else {
            line.contains(&query)
        };
        if found {
            results.push(line);
        }
    }
    results
}

pub fn run(config: &Config) -> Outcome {
    let mut output = String::new();
    let mut errors = Vec::new();
    let mut matched = false;
    let prefix = config.paths.len() > 1;
    for path in &config.paths {
        match fs::read_to_string(path) {
            Ok(contents) => {
                for line in matching_lines(&config.query, &contents, config.ignore_case) {
                    matched = true;
                    if prefix {
                        output.push_str(&format!("{path}:"));
                    }
                    output.push_str(line);
                    output.push('\n');
                }
            }
            Err(e) => errors.push(format!("{path}: {e}")),
        }
    }
    let exit_code = if !errors.is_empty() {
        2
    } else if matched {
        0
    } else {
        1
    };
    Outcome {
        output,
        errors,
        exit_code,
    }
}
