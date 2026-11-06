pub fn pop_all(mut stack: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(top) = stack.pop() {
        out.push(top);
    }
    out
}

pub fn pop_until(mut stack: Vec<i32>, stop: i32) -> (Vec<i32>, Vec<i32>) {
    let mut popped = Vec::new();
    while let Some(&top) = stack.last() {
        popped.push(top);
        stack.pop();
    }
    (popped, stack)
}
