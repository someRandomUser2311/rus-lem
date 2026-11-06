// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1200000D);
    for _ in 0..300 {
        let n = g.pick(8); let mut items = Vec::new(); for _ in 0..n { if g.pick(2) == 0 { let t = g.pick(5); let s = g.pick(32); let c = g.next() >> s; items.push(format!("c.add(Box::new(crate::Book {{ title: String::from(\"{t}\"), cents: {c} }}));")); } else { let s = g.pick(32); let c = g.next() >> s; let k = g.pick(5); items.push(format!("c.add(Box::new(crate::Bulk {{ name: String::from(\"b\"), cents: {c}, count: {k} }}));")); } }
        call(format!("{{ let mut c = crate::Cart::new(); {} (c.total_cents(), c.lines(), c.priciest()) }}", items.join(" ")));
    }
}
