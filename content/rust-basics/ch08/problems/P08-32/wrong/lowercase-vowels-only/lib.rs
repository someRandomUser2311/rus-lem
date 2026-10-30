pub fn pig_latin(text: &str) -> String {
    let mut out = String::new();
    for word in text.split_whitespace() {
        let mut first = ' ';
        let mut rest = word.len();
        for (i, c) in word.char_indices() {
            if i == 0 {
                first = c;
            } else {
                rest = i;
                break;
            }
        }
        if !out.is_empty() {
            out.push(' ');
        }
        if "aeiou".contains(first) {
            out.push_str(word);
            out.push_str("-hay");
        } else {
            out.push_str(&word[rest..]);
            out.push('-');
            out.push(first);
            out.push_str("ay");
        }
    }
    out
}
