fn main() {
    let mut g = Lcg(0x0300001E);
    for _ in 0..300 {
        let count = g.pick(21);
        let mut text = String::new();
        for _ in 0..count {
            let line = match g.pick(4) {
                0 => g.pick(1000001).to_string(),
                1 => "bad".to_string(),
                2 => String::new(),
                _ => "quit".to_string(),
            };
            text.push_str(&line);
            text.push('\n');
        }
        if g.pick(2) == 1 && !text.is_empty() {
            text.pop();
        }
        stdin(text);
    }
}
