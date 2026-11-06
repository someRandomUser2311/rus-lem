use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Sale {
    pub region: String,
    pub cents: u32,
    pub paid: bool,
}

pub fn revenue(sales: &[Sale], minimum: u64) -> Vec<(String, u64)> {
    let totals = sales
        .iter()
        .fold(HashMap::new(), |mut totals, s| {
            let cents = if s.paid { s.cents as u64 } else { 0 };
            *totals.entry(s.region.clone()).or_insert(0u64) += cents;
            totals
        });
    let mut out: Vec<(String, u64)> = totals
        .into_iter()
        .filter(|(_, total)| *total >= minimum)
        .collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    out
}
