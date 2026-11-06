//! Sales figures: totals per product and a printable report.

use std::collections::HashMap;

/// One sale: a product, how many, the price of one in cents, and whether it was paid.
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

/// The total value of the paid sales, in cents.
pub fn paid_total(sales: &[Sale]) -> u128 {
    sales.iter().filter(|s| s.paid).map(value).sum()
}

/// Paid sales grouped by product: highest total first, ties by product name.
///
/// # Examples
///
/// ```
/// use analysis_core::{product_totals, Sale};
///
/// let sales = vec![
///     Sale { product: String::from("tea"), quantity: 2, unit_cents: 125, paid: true },
///     Sale { product: String::from("cake"), quantity: 1, unit_cents: 305, paid: true },
/// ];
/// assert_eq!(
///     product_totals(&sales),
///     vec![(String::from("cake"), 305), (String::from("tea"), 250)]
/// );
/// ```
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

/// The first `n` products of [`product_totals`].
pub fn top_products(sales: &[Sale], n: usize) -> Vec<(String, u128)> {
    product_totals(sales).into_iter().take(n).collect()
}

/// One line per product, `product: $D.CC`, in [`product_totals`] order.
///
/// # Examples
///
/// ```
/// use analysis_core::{report, Sale};
///
/// let sales = vec![Sale { product: String::from("gum"), quantity: 1, unit_cents: 7, paid: true }];
/// assert_eq!(report(&sales), "gum: $0.07\n");
/// ```
pub fn report(sales: &[Sale]) -> String {
    product_totals(sales)
        .into_iter()
        .map(|(name, total)| format!("{name}: ${}.{:02}\n", total / 100, total % 100))
        .collect()
}
