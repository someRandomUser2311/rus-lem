use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Sale {
    pub product: String,
    pub quantity: u32,
    pub unit_cents: u32,
    pub paid: bool,
}

fn value(s: &Sale) -> u128 {
    s.quantity as u128 * s.unit_cents as u128
}

pub fn paid_total(sales: &[Sale]) -> u128 {
    sales.iter().filter(|s| s.paid).map(value).sum()
}

pub fn product_totals(sales: &[Sale]) -> Vec<(String, u128)> {
    let totals = sales
        .iter()
        .filter(|s| s.paid)
        .fold(HashMap::new(), |mut map, s| {
            *map.entry(s.product.clone()).or_insert(0u128) += value(s);
            map
        });
    let mut out: Vec<(String, u128)> = totals.into_iter().collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    out
}

pub fn top_products(sales: &[Sale], n: usize) -> Vec<(String, u128)> {
    product_totals(sales).into_iter().take(n).collect()
}

pub fn report(sales: &[Sale]) -> String {
    String::new()
}
