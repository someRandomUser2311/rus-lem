#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    BadLine(usize),
    UnknownKey(usize, String),
    Duplicate(usize, String),
    Missing(String),
    InvalidValue(usize, String),
}

#[derive(Debug, PartialEq)]
pub enum LoadError {
    Io(std::io::ErrorKind),
    Parse(ConfigError),
}

fn invalid(number: usize, key: &str) -> ConfigError {
    ConfigError::InvalidValue(number, key.to_string())
}

pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    let mut host: Option<String> = None;
    let mut port: Option<u16> = None;
    let mut workers: Option<u8> = None;
    let mut number = 0;
    for raw in text.lines() {
        number += 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut pieces = line.split('=');
        let key = pieces.next().ok_or(ConfigError::BadLine(number))?;
        let value = pieces.next().ok_or(ConfigError::BadLine(number))?.trim();
        if pieces.next().is_some() {
            return Err(ConfigError::BadLine(number));
        }
        let seen = match key {
            "host" => host.is_some(),
            "port" => port.is_some(),
            "workers" => workers.is_some(),
            _ => return Err(ConfigError::UnknownKey(number, key.to_string())),
        };
        if seen {
            return Err(ConfigError::Duplicate(number, key.to_string()));
        }
        if key == "host" {
            if value.is_empty() {
                return Err(invalid(number, key));
            }
            host = Some(value.to_string());
        } else if key == "port" {
            let n = value.parse::<u16>().ok().ok_or(invalid(number, key))?;
            if n == 0 {
                return Err(invalid(number, key));
            }
            port = Some(n);
        } else {
            let n = value.parse::<u8>().ok().ok_or(invalid(number, key))?;
            if n == 0 || n > 64 {
                return Err(invalid(number, key));
            }
            workers = Some(n);
        }
    }
    let host = host.ok_or(ConfigError::Missing("host".to_string()))?;
    let port = port.ok_or(ConfigError::Missing("port".to_string()))?;
    Ok(Config {
        host,
        port,
        workers: workers.unwrap_or(4),
    })
}

pub fn load_config(path: &str) -> Result<Config, LoadError> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => return Err(LoadError::Io(error.kind())),
    };
    match parse_config(&text) {
        Ok(config) => Ok(config),
        Err(error) => Err(LoadError::Parse(error)),
    }
}
