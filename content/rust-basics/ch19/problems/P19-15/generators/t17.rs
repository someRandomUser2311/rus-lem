// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1300000F);
    for _ in 0..300 {
        let n = g.pick(10); let mut evs = Vec::new(); for _ in 0..n { match g.pick(4) { 0 => { let u = g.pick(3); let a = g.pick(2) == 1; evs.push(format!("crate::Event::Login {{ user: String::from(\"{u}\"), admin: {a} }}")); } 1 => { let u = g.pick(3); evs.push(format!("crate::Event::Logout {{ user: String::from(\"{u}\") }}")); } 2 => { let u = g.pick(3); let c = g.pick(20) * 1000; evs.push(format!("crate::Event::Purchase {{ user: String::from(\"{u}\"), cents: {c} }}")); } _ => { let c = g.pick(700); evs.push(format!("crate::Event::Error {{ code: {c}, message: String::from(\"m\") }}")); } } }
        call(format!("crate::reduce(vec![{}])", evs.join(", ")));
    }
}
