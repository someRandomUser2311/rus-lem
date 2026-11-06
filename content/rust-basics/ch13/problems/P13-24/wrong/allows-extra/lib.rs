#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = args.next().ok_or("not enough arguments")?;
        let file_path = args.next().ok_or("not enough arguments")?;
        Ok(Config { query, file_path })
    }
}
