
fn hp(g: &mut Lcg, k: u32) -> u32 {
    (g.next() >> 16) % k
}
const A12: [&str; 12] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0"];
fn text(g: &mut Lcg) -> String {
    let len = hp(g, 25);
    let mut s = String::new();
    for _ in 0..len {
        s.push_str(A12[hp(g, 12) as usize]);
    }
    s
}
fn main() {
    let mut g = Lcg(0x07000009);
    for _ in 0..300 {
        const PRICE: [u32; 3] = [200, 350, 500];
        const NAME: [&str; 3] = ["Soup", "Salad", "Rice"];
        let mut lines: Vec<(usize, u32)> = Vec::new();
        match hp(&mut g, 4) {
            0 => {
                let n = hp(&mut g, 21);
                for _ in 0..n {
                    let d = hp(&mut g, 3) as usize;
                    let q = if hp(&mut g, 5) == 0 { 0 } else { hp(&mut g, 10001) };
                    lines.push((d, q));
                }
            }
            1 => {
                let d = hp(&mut g, 3) as usize;
                lines.push((d, u32::MAX / PRICE[d] + hp(&mut g, 2)));
            }
            2 => {
                for _ in 0..2 {
                    let d = hp(&mut g, 3) as usize;
                    lines.push((d, (u32::MAX / PRICE[d]) / 2 + 10 * hp(&mut g, 2)));
                }
            }
            _ => {
                let n = 1 + hp(&mut g, 20);
                for _ in 0..n {
                    let d = hp(&mut g, 3) as usize;
                    lines.push((d, g.next()));
                }
            }
        }
        let items: Vec<String> = lines
            .iter()
            .map(|(d, q)| format!("crate::Line{{dish:crate::Dish::{},quantity:{q}}}", NAME[*d]))
            .collect();
        let ty = if items.is_empty() { ": [crate::Line; 0]" } else { "" };
        call(format!(
            "{{let lines{ty}=[{}];(crate::quote(&lines),crate::receipt(&lines))}}",
            items.join(",")
        ));
    }
}
