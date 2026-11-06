#[derive(Debug, PartialEq)]
pub enum Range {
    One(usize),
    Span(usize, usize),
    From(usize),
}

/// One line number: ASCII digits only, fits a `usize`, not 0.
fn number(text: &str) -> Result<usize, String> {
    if text.is_empty() {
        return Err(format!("bad line number: {text}"));
    }
    for c in text.chars() {
        if !c.is_ascii_digit() {
            return Err(format!("bad line number: {text}"));
        }
    }
    let n = match text.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(format!("bad line number: {text}")),
    };
    if n == 0 {
        return Err(String::from("line numbers start at 1"));
    }
    Ok(n)
}

pub fn parse_spec(spec: &str) -> Result<Vec<Range>, String> {
    let mut ranges = Vec::new();
    for raw in spec.split(',') {
        let item = raw.trim();
        if item.is_empty() {
            return Err(String::from("empty range"));
        }
        if item.matches('-').count() > 1 || item == "-" {
            return Err(format!("bad range: {item}"));
        }
        match item.split_once('-') {
            None => ranges.push(Range::One(number(item)?)),
            Some((a, b)) => {
                let start = if a.is_empty() { 1 } else { number(a)? };
                if b.is_empty() {
                    ranges.push(Range::From(start));
                } else {
                    let end = number(b)?;
                    if start > end {
                        return Err(format!("backwards range: {item}"));
                    }
                    ranges.push(Range::Span(start, end));
                }
            }
        }
    }
    Ok(ranges)
}

pub fn select(ranges: &[Range], contents: &str) -> String {
    let mut lines = Vec::new();
    for line in contents.lines() {
        lines.push(line);
    }
    let mut out = String::new();
    for range in ranges {
        let (first, last) = match range {
            Range::One(n) => (*n, *n),
            Range::Span(a, b) => (*a, *b),
            Range::From(a) => (*a, lines.len()),
        };
        let last = last;
        if first > lines.len() {
            continue;
        }
        let mut n = first;
        while n <= last {
            out.push_str(&format!("{n}: {}\n", lines[n - 1]));
            n += 1;
        }
    }
    out
}
