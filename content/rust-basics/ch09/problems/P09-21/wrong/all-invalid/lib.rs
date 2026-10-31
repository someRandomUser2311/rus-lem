use std::num::IntErrorKind;

pub fn recover_tokens(text: &str) -> (Vec<u32>, usize, usize) {
    let mut values = Vec::new();
    let mut invalid = 0;
    let mut overflow = 0;
    for token in text.split_whitespace() {
        match token.parse::<u32>() {
            Ok(n) => values.push(n),
            Err(e) => match e.kind() {
                IntErrorKind::NegOverflow => overflow += 1,
                _ => invalid += 1,
            },
        }
    }
    (values, invalid, overflow)
}
