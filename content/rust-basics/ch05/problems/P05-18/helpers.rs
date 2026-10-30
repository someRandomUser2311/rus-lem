/// Runs `|`-separated operations on `Playlist::new(name)` and logs each result with the
/// name, active song ids and count afterwards; then the consuming summary. Operations:
/// `a<id>` add, `c` clear, `r<name>` rename, `n` count.
fn drive(name: &str, ops: &str) -> String {
    let mut p = Playlist::new(name);
    let mut log = String::new();
    for op in ops.split('|').filter(|o| !o.is_empty()) {
        let (code, arg) = op.split_at(1);
        let result = match code {
            "a" => format!("{:?}", p.add(arg.parse().unwrap_or(0))),
            "c" => {
                p.clear();
                "()".to_string()
            }
            "r" => format!("{:?}", p.rename(arg)),
            _ => format!("{:?}", p.count()),
        };
        log.push_str(&format!(
            "{op:?} -> {result} {:?} {:?} {}\n",
            p.name,
            &p.songs[..p.len.min(5)],
            p.count()
        ));
    }
    log.push_str(&p.summary());
    log
}
