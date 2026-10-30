pub fn calculate(a: i32, b: i32, operator: char) -> (bool, i64) {
    let x = i64::from(a);
    let y = i64::from(b);
    if operator == '+' {
        (true, x + y)
    } else if operator == '-' {
        (true, x - y)
    } else if operator == '*' {
        (true, x * y)
    } else if (operator == '/' || operator == '%') && y == 0 {
        (false, 0)
    } else if operator == '/' {
        (true, x / y)
    } else if operator == '%' {
        (true, x % y)
    } else {
        (false, 0)
    }
}
