use crate::text;

pub fn render(labels: &[Option<&str>]) -> String {
    let mut out = String::new();
    let mut number = 0;
    for label in labels {
        let Some(raw) = label else {
            continue;
        };
        let normalized = text::normalize(raw);
        if normalized.is_empty() {
            continue;
        }
        number += 1;
        out.push_str(&format!("{number}. {normalized}\n"));
    }
    out
}
