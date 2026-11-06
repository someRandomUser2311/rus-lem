pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x.saturating_add(n)
}

pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |name| format!("{greeting}, {name}!")
}

pub fn pick(op: &str) -> Option<Box<dyn Fn(i32, i32) -> i32>> {
    match op {
        "add" => Some(Box::new(|a: i32, b: i32| a.saturating_add(b))),
        "sub" => Some(Box::new(|a: i32, b: i32| a.saturating_sub(b))),
        "max" => Some(Box::new(|a: i32, b: i32| a.max(b))),
        _ => None,
    }
}

pub fn compose(f: Box<dyn Fn(i32) -> i32>, g: Box<dyn Fn(i32) -> i32>) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| f(g(x)))
}
