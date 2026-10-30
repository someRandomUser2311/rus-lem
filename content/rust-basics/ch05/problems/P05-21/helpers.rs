/// Runs `|`-separated operations on `Ledger::new()` and logs each result with the seat
/// arrays and the occupied count afterwards. Operations: `r<seat>,<id>` reserve,
/// `c<seat>` cancel, `t<from>,<to>` transfer.
fn drive(ops: &str) -> String {
    let mut l = Ledger::new();
    let mut log = String::new();
    for op in ops.split('|').filter(|o| !o.is_empty()) {
        let (code, arg) = op.split_at(1);
        let mut n = arg.split(',').map(|x| x.parse::<u64>().unwrap_or(0));
        let a = n.next().unwrap_or(0);
        let b = n.next().unwrap_or(0);
        let ok = match code {
            "r" => l.reserve(a as usize, b as u32),
            "c" => l.cancel(a as usize),
            _ => l.transfer(a as usize, b as usize),
        };
        log.push_str(&format!("{op} -> {ok} {:?} {:?} {}\n", l.used, l.names, l.occupied()));
    }
    log
}
