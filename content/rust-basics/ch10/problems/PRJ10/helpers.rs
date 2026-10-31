/// The course generator with high-bit picks: pick(k) = (next() >> 16) % k.
struct Prj10Rng(u32);
impl Prj10Rng {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0
    }
    fn pick(&mut self, k: u32) -> u32 {
        (self.next() >> 16) % k
    }
    fn text(&mut self) -> String {
        const A: [char; 12] = ['a', 'Z', 'é', '東', '😀', 'e', '\u{301}', ' ', '\t', '\n', '\u{3000}', '\0'];
        let n = self.pick(25);
        let mut s = String::new();
        for _ in 0..n {
            s.push(A[self.pick(12) as usize]);
        }
        s
    }
}

/// A test entry with no derives: neither Clone nor Copy.
struct Prj10Entry {
    id: usize,
    name: String,
    score: i64,
}
impl crate::Ranked for Prj10Entry {
    fn points(&self) -> i64 {
        self.score
    }
    fn label(&self) -> &str {
        &self.name
    }
}

/// Case `want` of the seeded suite: the ids `top`, `best` and the view's `first` return,
/// then the view's summary, the report and `len`.
fn prj10_case(want: usize) -> String {
    let mut r = Prj10Rng(0x0A000101);
    for case in 0..=want {
        let n = r.pick(9) as usize;
        let mut b = crate::Leaderboard::new();
        for id in 0..n {
            let kind = r.pick(5);
            let score = match kind {
                0 => i64::MIN,
                1 => i64::MAX,
                2 | 3 => r.pick(5) as i64 - 2,
                _ => r.next() as i64 - 2147483648,
            };
            let name = r.text();
            b.add(Prj10Entry { id, name, score });
        }
        let limit = match r.pick(4) {
            0 => 0,
            1 => 1 + r.pick(n as u32 + 1) as usize,
            2 => n,
            _ => usize::MAX,
        };
        if case < want {
            continue;
        }
        let mut top = Vec::new();
        for e in b.top(limit) {
            top.push(e.id);
        }
        let best = b.best().map(|e| e.id);
        let view = b.view(limit);
        let first = view.first().map(|e| e.id);
        return format!("top {top:?} best {best:?} first {first:?} view {:?} report {:?} len {}", view.summary(), b.report(), b.len());
    }
    unreachable!()
}
