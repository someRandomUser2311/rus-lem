// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x12010000);
    for _ in 0..300 {
        let cmds = ["add a", "add é", "review", "approve", "reject", "status", "show"]; let n = g.pick(10); let ls: Vec<&str> = (0..n).map(|_| cmds[g.pick(7) as usize]).collect(); let text = ls.join("\n");
        call(format!("crate::run({text:?})"));
    }
}
