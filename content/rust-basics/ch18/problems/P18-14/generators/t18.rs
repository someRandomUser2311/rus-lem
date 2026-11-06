// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1200000E);
    for _ in 0..300 {
        let n = g.pick(12); let mut cs = Vec::new(); for _ in 0..n { match g.pick(4) { 0 => cs.push(String::from("p.request_review();")), 1 => { let r = g.pick(3); cs.push(format!("p.approve({r});")); } 2 => cs.push(String::from("p.reject();")), _ => cs.push(String::from("p.add_text(\"t\");")) } }
        call(format!("{{ let mut p = crate::Post::new(); {} (p.status(), p.approvers(), p.content().to_string()) }}", cs.join(" ")));
    }
}
