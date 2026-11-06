pub fn port(line: &str) -> Result<u16, String> {
    let Some((key, value)) = line.split_once('=') else {
        return Err(String::from("missing ="));
    };
    let "port" = key else {
        return Err(format!("unknown key: {}", key.trim()));
    };
    let Ok(n) = value.trim().parse::<u16>() else {
        return Err(format!("bad port: {}", value.trim()));
    };
    Ok(n)
}
