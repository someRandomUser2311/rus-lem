use std::io::BufRead;

pub fn read_head(reader: &mut impl BufRead) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    let mut total = 0;
    loop {
        let mut line = String::new();
        let n = reader
            .read_line(&mut line)
            .map_err(|_| String::from("invalid UTF-8"))?;
        if n == 0 {
            return Err(String::from("incomplete request"));
        }
        total += n;
        let text = line.trim_end_matches(['\r', '\n']);
        if text.is_empty() {
            return Ok(lines);
        }
        lines.push(text.to_string());
    }
}

pub fn request_line(lines: &[String]) -> Option<(String, String, String)> {
    let first = lines.first()?;
    let [method, path, version] = first.split(' ').collect::<Vec<_>>()[..] else {
        return None;
    };
    Some((method.to_string(), path.to_string(), version.to_string()))
}
