// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x10000008);
    for _ in 0..300 {
        let gs = g.pick(6); let mut out = Vec::new(); for _ in 0..gs { let n = g.pick(6); let v: Vec<u32> = (0..n).map(|_| { let s = g.pick(32); g.next() >> s }).collect(); out.push(format!("vec!{v:?}")); }
        call(format!("{{ let gs: Vec<Vec<u32>> = vec![{}]; crate::group_totals(gs) }}", out.join(", ")));
    }
}
