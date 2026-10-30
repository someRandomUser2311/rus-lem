pub fn calculate(a: i32, b: i32, operator: char) -> (bool, i64) {
    let x = a as i64;
    let y = b as i64;
    if operator == '+' {
        (true, x + y)
    } else if operator == '-' {
        (true, x - y)
    } else if operator == '*' {
        (true, a.wrapping_mul(b) as i64)
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
