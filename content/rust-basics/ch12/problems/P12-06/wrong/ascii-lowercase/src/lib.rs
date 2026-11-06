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
    let query = query.to_ascii_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_ascii_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn ignore_case_from(value: Option<&str>) -> bool {
    match value {
        None => false,
        Some(v) => {
            let v = v.trim().to_lowercase();
            !(v.is_empty() || v == "0" || v == "false" || v == "no" || v == "off")
        }
    }
}
