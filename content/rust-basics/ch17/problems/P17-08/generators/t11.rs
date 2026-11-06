// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x11000008);
    for _ in 0..300 {
        let a = ["a", "b", "é", "東", " "]; let n = g.pick(8); let mut ts = Vec::new(); for _ in 0..n { let k = g.pick(4); let mut s = String::new(); for _ in 0..k { s.push_str(a[g.pick(5) as usize]); } ts.push(format!("String::from({s:?})")); }
        call(format!("trpl::block_on(crate::task_messages(vec![{}]))", ts.join(", ")));
    }
}
