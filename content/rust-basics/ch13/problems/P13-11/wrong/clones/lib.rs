pub fn mark(values: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for mut value in values.into_iter() {
        let mut copy = value.clone();
        copy.push('!');
        out.push(copy);
    }
    out
}
