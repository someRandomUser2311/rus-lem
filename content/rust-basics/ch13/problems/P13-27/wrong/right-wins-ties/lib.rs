pub fn merge(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    let mut a = left.into_iter().peekable();
    let mut b = right.into_iter().peekable();
    let mut out = Vec::new();
    loop {
        let take_left = match (a.peek(), b.peek()) {
            (Some(x), Some(y)) => x < y,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => break,
        };
        let next = if take_left { a.next() } else { b.next() };
        if let Some(s) = next {
            out.push(s);
        }
    }
    out
}
