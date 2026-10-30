/// Runs `|`-separated operations on two new accounts A and B and logs each result with
/// both balances afterwards. Operations: `d<cents>` A deposits, `w<cents>` A withdraws,
/// `t<cents>` A transfers to B.
fn drive(ops: &str) -> String {
    let mut a = BankAccount::new("A");
    let mut b = BankAccount::new("B");
    let mut log = String::new();
    for op in ops.split('|').filter(|o| !o.is_empty()) {
        let (code, arg) = op.split_at(1);
        let cents: u64 = arg.parse().unwrap_or(0);
        let ok = match code {
            "d" => a.deposit(cents),
            "w" => a.withdraw(cents),
            _ => a.transfer_to(&mut b, cents),
        };
        log.push_str(&format!("{op} -> {ok} {} {}\n", a.balance_cents, b.balance_cents));
    }
    log
}
