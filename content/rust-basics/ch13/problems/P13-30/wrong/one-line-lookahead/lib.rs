pub struct Words<'a> {
    lines: std::slice::Iter<'a, String>,
    current: std::str::SplitWhitespace<'a>,
}

impl<'a> Words<'a> {
    pub fn new(lines: &'a [String]) -> Self {
        Self {
            lines: lines.iter(),
            current: "".split_whitespace(),
        }
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if let Some(word) = self.current.next() {
            return Some(word);
        }
        match self.lines.next() {
            Some(line) => self.current = line.split_whitespace(),
            None => return None,
        }
        self.current.next()
    }
}

pub fn collect_words(lines: &[String]) -> Vec<&str> {
    Words::new(lines).collect()
}
