#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn from_values(values: &[i32]) -> List {
    let mut list = List::Nil;
    for v in values.iter() {
        list = List::Cons(*v, Box::new(list));
    }
    list
}

pub fn list_values(list: &List) -> Vec<i32> {
    let mut out = Vec::new();
    let mut current = list;
    while let List::Cons(v, next) = current {
        out.push(*v);
        current = next;
    }
    out
}
