// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D010001);
    let titles = ["The Cat", "A Cat", "Cat", "Dune", "émile"];
    let authors = ["x", "X", " x ", "y"];
    let actions = ["List", "List", "Count", "Authors", "Decades", "Find(String::from(\"cat\"))"];
    for _ in 0..300 {
        let n = g.pick(8);
        let mut text = String::new();
        for _ in 0..n {
            let t = titles[g.pick(5) as usize];
            let a = authors[g.pick(4) as usize];
            let y = g.pick(3000);
            if g.pick(10) == 0 { text.push_str("bad line\n"); } else { text.push_str(&format!("{t}|{a}|{y}\n")); }
        }
        let k = g.pick(6) as usize;
        let by_year = k == 1;
        call(format!("crate::cli::run(&crate::cli::Config {{ catalog_path: String::from(\"cat.txt\"), action: crate::cli::Action::{}, by_year: {by_year} }}, {text:?})", actions[k]));
    }
}
