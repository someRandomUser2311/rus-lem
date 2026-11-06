// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D010000);
    let products = ["tea", "cake", "Tea", "", "東"];
    for _ in 0..300 {
        let n = g.pick(10);
        let sales: Vec<String> = (0..n).map(|_| {
            let p = products[g.pick(5) as usize];
            let s1 = g.pick(32);
            let q = g.next() >> s1;
            let s2 = g.pick(32);
            let u = g.next() >> s2;
            let paid = g.pick(3) != 0;
            format!("crate::Sale {{ product: String::from({p:?}), quantity: {q}, unit_cents: {u}, paid: {paid} }}")
        }).collect();
        let k = g.pick(6);
        call(format!("{{ let s: Vec<crate::Sale> = vec![{}]; (crate::paid_total(&s), crate::product_totals(&s), crate::top_products(&s, {k}), crate::report(&s)) }}", sales.join(", ")));
    }
}
