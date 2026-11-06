// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x14000011);
    for _ in 0..300 {
        let n = g.pick(6); let k = g.pick(5); let mut ups = Vec::new(); for _ in 0..k { let i = g.pick(8); let v = g.pick(100); ups.push(format!("({i}, {v})")); }
        call(format!("{{ let mut v = vec![0i32; {n}]; let r = crate::scatter(&mut v, &[{}]); (r, v) }}", ups.join(", ")));
    }
}
