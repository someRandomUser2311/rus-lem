// 300 documents drawn as the spec's Random tests paragraph describes (seed 0x09020000).
// Draws use the high bits: pick(k) = (next() >> 16) % k.
fn hp(r: &mut Lcg, k: u32) -> u32 {
    (r.next() >> 16) % k
}

const B: [&str; 3] = ["", "  ", "\u{3000}"];
const CM: [&str; 2] = ["# c", "  #host=x"];
const HK: [&str; 3] = ["host", " host ", "Host"];
const SG: [&str; 2] = ["=", " = "];
const HV: [&str; 5] = ["localhost", "", " a b ", "東京😀", "x=y"];
const PV: [&str; 5] = ["0", "65536", "+80", "08", "x"];
const WV: [&str; 5] = ["0", "65", "256", "+7", ""];
const UK: [&str; 3] = ["user=ann", "=5", "ｈｏｓｔ=x"];
const BL: [&str; 3] = ["host", "host==x", "port 80"];
const G: [&str; 2] = ["host=h", "port=80"];
const EN: [&str; 2] = ["\n", "\r\n"];
fn main() {
    let mut r = Lcg(0x09020000);
    for _ in 0..300 {
        let count = 2 + hp(&mut r, 5);
        let mut lines = Vec::new();
        for _ in 0..count {
            let line = match hp(&mut r, 12) {
                0 => B[hp(&mut r, 3) as usize].to_string(),
                1 => CM[hp(&mut r, 2) as usize].to_string(),
                2 | 3 => {
                    let k = HK[hp(&mut r, 3) as usize];
                    let s = SG[hp(&mut r, 2) as usize];
                    format!("{k}{s}{}", HV[hp(&mut r, 5) as usize])
                }
                4 | 5 => {
                    let s = SG[hp(&mut r, 2) as usize];
                    let v = hp(&mut r, 6);
                    let v = if v == 0 { (1 + hp(&mut r, 65535)).to_string() } else { PV[v as usize - 1].to_string() };
                    format!("port{s}{v}")
                }
                6 => {
                    let s = SG[hp(&mut r, 2) as usize];
                    let v = hp(&mut r, 6);
                    let v = if v == 0 { (1 + hp(&mut r, 64)).to_string() } else { WV[v as usize - 1].to_string() };
                    format!("workers{s}{v}")
                }
                7 => UK[hp(&mut r, 3) as usize].to_string(),
                8 => BL[hp(&mut r, 3) as usize].to_string(),
                _ => G[hp(&mut r, 2) as usize].to_string(),
            };
            lines.push(line);
        }
        let end = EN[hp(&mut r, 2) as usize];
        let mut text = lines.join(end);
        if hp(&mut r, 2) == 1 {
            text.push_str(end);
        }
        call(format!("crate::parse_config({text:?})"));
    }
}
