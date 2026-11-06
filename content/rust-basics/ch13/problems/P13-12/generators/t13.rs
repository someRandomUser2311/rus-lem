// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D00000C);
    for _ in 0..300 {
        let (s, e, st) = match g.pick(3) {
            0 => (g.pick(50), g.pick(50), g.pick(8)),
            1 => (u32::MAX - g.pick(60), u32::MAX - g.pick(10), g.pick(20)),
            _ => (g.pick(1000), u32::MAX - g.pick(1000), u32::MAX / 4 + g.pick(1000)),
        };
        call(format!("crate::step_values({s}, {e}, {st})"));
    }
}
