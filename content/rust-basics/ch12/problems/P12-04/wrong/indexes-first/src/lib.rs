#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let query = args[1].clone();
        let file_path = args[2].clone();
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 3 {
            return Err("too many arguments");
        }
        Ok(Config { query, file_path })
    }
}
