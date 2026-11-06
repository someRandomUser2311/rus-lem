use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Person {
    pub name: String,
    parent: RefCell<Weak<Person>>,
    children: RefCell<Vec<Rc<Person>>>,
}

pub fn person(name: &str) -> Rc<Person> {
    Rc::new(Person {
        name: name.to_string(),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    })
}

fn parent_of(p: &Rc<Person>) -> Option<Rc<Person>> {
    p.parent.borrow().upgrade()
}

pub fn attach(parent: &Rc<Person>, child: &Rc<Person>) -> Result<(), &'static str> {
    let mut up = Some(Rc::clone(parent));
    while let Some(p) = up {
        if Rc::ptr_eq(&p, child) {
            return Err("cycle");
        }
        up = parent_of(&p);
    }
    match parent_of(child) {
        Some(current) if Rc::ptr_eq(&current, parent) => return Ok(()),
        Some(_) => return Err("parented"),
        None => {}
    }
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
    Ok(())
}

pub fn parent_name(person: &Rc<Person>) -> Option<String> {
    parent_of(person).map(|p| p.name.clone())
}

pub fn children_names(person: &Rc<Person>) -> Vec<String> {
    let mut names: Vec<String> = person
        .children
        .borrow()
        .iter()
        .map(|c| c.name.clone())
        .collect();
    names.sort();
    names
}

pub fn detach(parent: &Rc<Person>, child: &Rc<Person>) -> bool {
    match parent_of(child) {
        Some(current) if Rc::ptr_eq(&current, parent) => {
            parent.children.borrow_mut().retain(|c| !Rc::ptr_eq(c, child));
            *child.parent.borrow_mut() = Weak::new();
            true
        }
        _ => false,
    }
}

pub fn ancestors(person: &Rc<Person>) -> Vec<String> {
    let mut out = Vec::new();
    let mut up = parent_of(person);
    while let Some(p) = up {
        out.push(p.name.clone());
        up = parent_of(&p);
    }
    out
}
