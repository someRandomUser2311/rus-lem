fn has_word(line: &str, query: &str) -> bool {
    let mut word = String::new();
    for c in line.chars() {
        if c.is_alphanumeric() {
            word.push(c);
        } else {
            if word == query {
                return true;
            }
            word.clear();
        }
    }
    false
}

pub fn search_whole_word<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    if query.is_empty() {
        return results;
    }
    for line in contents.lines() {
        if has_word(line, query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_a_whole_word() {
        let contents = "the cat sat\nconcatenate\ncat, dog";
        assert_eq!(vec!["the cat sat", "cat, dog"], search_whole_word("cat", contents));
    }

    #[test]
    fn finds_the_last_word_of_a_line() {
        assert_eq!(vec!["a cat"], search_whole_word("cat", "a cat\ncats"));
    }

    #[test]
    fn underscores_and_hyphens_separate_words() {
        assert_eq!(
            vec!["my_cat", "cat-like"],
            search_whole_word("cat", "my_cat\ncat-like\ncatlike")
        );
    }

    #[test]
    fn is_case_sensitive() {
        assert_eq!(vec!["Cat"], search_whole_word("Cat", "cat\nCat"));
    }

    #[test]
    fn an_empty_query_matches_nothing() {
        assert_eq!(Vec::<&str>::new(), search_whole_word("", "a\n\nb"));
    }

    #[test]
    fn a_line_is_returned_once() {
        assert_eq!(vec!["cat cat"], search_whole_word("cat", "cat cat"));
    }
}
