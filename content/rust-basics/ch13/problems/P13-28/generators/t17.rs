// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D00001C);
    for _ in 0..300 {
        let start = match g.pick(3) {
            0 => format!("{}", g.pick(100)),
            1 => format!("i64::MAX - {}", g.pick(10)),
            _ => format!("i64::MIN + {}", g.pick(10)),
        };
        let n = g.pick(8);
        let d: Vec<i64> = (0..n).map(|_| g.pick(21) as i64 - 10).collect();
        let limit = g.pick(120) as i64;
        call(format!("{{ let mut b: i64 = {start}; let mut calls = 0; let d: Vec<i64> = vec!{d:?}; let r = crate::transact(&mut b, &d, |v| {{ calls += 1; if v > {limit} {{ Err(String::from(\"over {limit}\")) }} else {{ Ok(v) }} }}); (r, b, calls) }}"));
    }
}
