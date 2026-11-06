use std::rc::Rc;

pub fn owner_counts(extra: usize) -> (usize, usize) {
    let value = Rc::new(7);
    let during = Rc::strong_count(&value);
    let copies: Vec<Rc<i32>> = (0..extra).map(|_| Rc::clone(&value)).collect();
    drop(copies);
    (during, Rc::strong_count(&value))
}
