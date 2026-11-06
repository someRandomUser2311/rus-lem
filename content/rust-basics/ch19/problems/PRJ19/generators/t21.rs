// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x13010000);
    for _ in 0..300 {
        let keys = ["a", "b", "c"]; let n = g.pick(8); let mut ls = Vec::new(); for _ in 0..n { let line = match g.pick(8) { 0 => { let k = keys[g.pick(3) as usize]; let v = g.pick(21) as i64 - 10; format!("set {k} {v}") } 1 => format!("get {}", keys[g.pick(3) as usize]), 2 => format!("del {}", keys[g.pick(3) as usize]), 3 => format!("incr {}", keys[g.pick(3) as usize]), 4 => { let k = keys[g.pick(3) as usize]; let v = g.pick(21) as i64 - 10; format!("incr {k} {v}") } 5 => String::from("list"), 6 => { let a = g.pick(21) as i64 - 10; let b = g.pick(21) as i64 - 10; format!("count {a} to {b}") } _ => String::from("bogus") }; ls.push(line); } let script = ls.join("\n");
        call(format!("crate::run({script:?})"));
    }
}
