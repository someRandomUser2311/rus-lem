use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn shared_tail(x: i32, y: i32, tail: i32) -> (Rc<List>, Rc<List>) {
    let t = Rc::new(List::Cons(tail, Rc::new(List::Nil)));
    (
        Rc::new(List::Cons(x, Rc::new(List::Cons(tail, Rc::new(List::Nil))))),
        Rc::new(List::Cons(y, t)),
    )
}

pub fn inspect(a: &Rc<List>, b: &Rc<List>) -> Option<(i32, i32, i32, bool, usize)> {
    match (&**a, &**b) {
        (List::Cons(x, ta), List::Cons(y, tb)) => match &**ta {
            List::Cons(z, _) => Some((*x, *y, *z, Rc::ptr_eq(ta, tb), Rc::strong_count(ta))),
            List::Nil => None,
        },
        _ => None,
    }
}
