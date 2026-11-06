// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1300000E);
    for _ in 0..300 {
        let n = g.pick(6); let mut msg: Vec<u8> = Vec::new(); for _ in 0..n { let b = if g.pick(2) == 0 { g.pick(6) } else { g.pick(256) }; msg.push(b as u8); }
        call(format!("{{ let msg: Vec<u8> = vec!{msg:?}; (crate::parse(&msg), crate::respond(&msg)) }}"));
    }
}
