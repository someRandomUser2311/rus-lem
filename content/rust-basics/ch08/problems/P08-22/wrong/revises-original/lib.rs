pub fn editions(texts: &[String]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for t in texts {
        let mut revised = t.clone();
        revised.push('!');
        out.push((revised.clone(), revised));
    }
    out
}
