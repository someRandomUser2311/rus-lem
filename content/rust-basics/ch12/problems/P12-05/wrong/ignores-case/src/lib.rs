pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn two_results() {
        let contents = "to be\nor not\nto see";
        assert_eq!(vec!["to be", "to see"], search("to", contents));
    }

    #[test]
    fn no_results() {
        assert_eq!(Vec::<&str>::new(), search("xyz", "abc\ndef"));
    }
}
