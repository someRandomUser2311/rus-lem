/// The generated suite (seed 0x08010000): case `case` of 300, each a fresh directory driven
/// by 20 drawn operations. The draws form one stream across cases, so this resumes where
/// the previous case stopped (or replays from the start when called out of order).
/// Returns one line per step: the operation's result, then the whole `all()` snapshot.
fn prj08_case(case: u32) -> String {
    use std::cell::Cell;
    thread_local! {
        static AT: Cell<(u32, u32)> = const { Cell::new((0, 0x08010000)) };
    }
    struct G(u32);
    impl G {
        fn pick(&mut self, k: u32) -> usize {
            self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
            ((self.0 >> 16) % k) as usize
        }
    }
    const N: [&str; 8] = ["Ada", " Ada ", "ada", "é", "e\u{301}", "東京", "\u{3000}😀", ""];
    const D: [&str; 6] = ["Ops", " Ops\t", "ops", "営業", "\u{3000}", ""];
    const W: [&str; 5] = ["Ada", "é", "東京", "e\u{301}", "Sally Ann"];
    const E: [&str; 4] = ["Ops", "Back to School", "営業", "ops"];
    const J: [&str; 3] = ["", "add Ada to Ops", "All all"];
    let (mut next, mut state) = AT.with(|a| a.get());
    if next > case {
        next = 0;
        state = 0x08010000;
    }
    let mut g = G(state);
    let mut log = String::new();
    while next <= case {
        let mut d = crate::Directory::new();
        log.clear();
        for step in 0..20 {
            let res = match g.pick(4) {
                0 => {
                    let n = N[g.pick(8)];
                    let dep = D[g.pick(6)];
                    format!("add({n:?}, {dep:?}) = {:?}", d.add(n, dep))
                }
                1 => {
                    let n = N[g.pick(8)];
                    format!("remove({n:?}) = {:?}", d.remove(n))
                }
                2 => {
                    let dep = D[g.pick(6)];
                    format!("department({dep:?}) = {:?}", d.department(dep))
                }
                _ => {
                    let line = match g.pick(6) {
                        0 => {
                            let w = W[g.pick(5)];
                            let e = E[g.pick(4)];
                            format!("Add {w} to {e}")
                        }
                        1 => format!("List {}", E[g.pick(4)]),
                        2 => format!("Remove {}", W[g.pick(5)]),
                        3 => "All".to_string(),
                        4 => format!("Add {}", W[g.pick(5)]),
                        _ => J[g.pick(3)].to_string(),
                    };
                    format!("command({line:?}) = {:?}", d.command(&line))
                }
            };
            log.push_str(&format!("{step}: {res} | all = {:?}\n", d.all()));
        }
        next += 1;
    }
    AT.with(|a| a.set((next, g.0)));
    log
}
