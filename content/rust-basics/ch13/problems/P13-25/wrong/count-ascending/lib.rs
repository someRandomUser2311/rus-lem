use std::collections::HashMap;

pub fn counts(words: &[String]) -> Vec<(String, usize)> {
    let map = words.iter().fold(HashMap::new(), |mut map, w| {
        *map.entry(w.clone()).or_insert(0usize) += 1;
        map
    });
    let mut out: Vec<(String, usize)> = map.into_iter().collect();
    out.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    out
}
