pub fn run(program: &str) -> Result<Vec<i64>, String> {
    let mut stack: Vec<i64> = Vec::new();
    for token in program.split_whitespace() {
        if let Ok(n) = token.parse::<i64>() {
            stack.push(n);
            continue;
        }
        let len = stack.len();
        match (token, stack.as_slice()) {
            ("+" | "-" | "*", [.., a, b]) => {
                let (a, b) = (*a, *b);
                let result = match token {
                    "+" => a.checked_add(b),
                    "-" => a.checked_sub(b),
                    _ => a.checked_mul(b),
                };
                let value = result.unwrap_or(0);
                stack.truncate(len - 2);
                stack.push(value);
            }
            ("dup", [.., a]) => {
                let a = *a;
                stack.push(a);
            }
            ("swap", [.., _, _]) => stack.swap(len - 2, len - 1),
            ("drop", [_, ..]) => {
                stack.pop();
            }
            ("+" | "-" | "*" | "dup" | "swap" | "drop", _) => {
                return Err(format!("stack underflow at {token}"));
            }
            _ => return Err(format!("unknown token: {token}")),
        }
    }
    Ok(stack)
}
