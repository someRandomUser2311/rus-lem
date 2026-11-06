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
    Vec::new()
}

pub fn top_products(sales: &[Sale], n: usize) -> Vec<(String, u128)> {
    Vec::new()
}

pub fn report(sales: &[Sale]) -> String {
    String::new()
}
