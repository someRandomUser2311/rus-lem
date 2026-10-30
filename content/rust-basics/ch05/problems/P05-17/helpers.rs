/// Runs `|`-separated editor operations from an empty Editor and logs each one with the
/// backspace result (if any), the text and the cursor afterwards. Operations: `i<c>`
/// insert, `t<text>` type_str, `b` backspace, `l` left, `r` right, `h` home, `e` end,
/// `v` inspect.
fn drive(ops: &str) -> String {
    let mut e = Editor::new();
    let mut log = String::new();
    for op in ops.split('|').filter(|o| !o.is_empty()) {
        let (code, arg) = op.split_at(1);
        let back = match code {
            "i" => {
                e.insert(arg.chars().next().unwrap_or(' '));
                None
            }
            "t" => {
                e.type_str(arg);
                None
            }
            "b" => Some(e.backspace()),
            "l" => {
                e.left();
                None
            }
            "r" => {
                e.right();
                None
            }
            "h" => {
                e.home();
                None
            }
            "e" => {
                e.end();
                None
            }
            _ => None,
        };
        log.push_str(&format!("{op:?} -> {back:?} {:?} {}\n", e.text, e.cursor));
    }
    log
}
