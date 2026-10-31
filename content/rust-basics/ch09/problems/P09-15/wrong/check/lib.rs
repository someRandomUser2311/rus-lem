pub fn must_pop(stack: &mut Vec<i32>) -> i32 {
    stack.pop().expect("internal stack must contain a value")
}

pub fn sum_top(stack: &mut Vec<i32>, count: usize) -> Result<i64, String> {
    let len = stack.len();
    if count >= len && count > 0 {
        return Err(format!("wanted {count}, have {len}"));
    }
    let mut total: i64 = 0;
    for _ in 0..count {
        total += must_pop(stack) as i64;
    }
    Ok(total)
}
