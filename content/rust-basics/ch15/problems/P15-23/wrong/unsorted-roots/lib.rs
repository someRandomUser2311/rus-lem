use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Person {
    pub name: String,
    pub parent: RefCell<Weak<Person>>,
    pub children: RefCell<Vec<Rc<Person>>>,
}

fn path(person: &Rc<Person>) -> String {
    let mut parts = vec![person.name.clone()];
    let mut current = person.parent.borrow().upgrade();
    while let Some(p) = current {
        parts.push(p.name.clone());
        current = p.parent.borrow().upgrade();
    }
    parts.reverse();
    parts.join("/")
}

pub fn forest(names: &[&str], parents: &[Option<usize>]) -> (Vec<String>, Vec<String>, bool) {
    let people: Vec<Rc<Person>> = names
        .iter()
        .map(|n| {
            Rc::new(Person {
                name: n.to_string(),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        })
        .collect();
    for (i, parent) in parents.iter().enumerate() {
        if let Some(p) = parent
            && *p < i
        {
            people[*p].children.borrow_mut().push(Rc::clone(&people[i]));
            *people[i].parent.borrow_mut() = Rc::downgrade(&people[*p]);
        }
    }
    let mut roots: Vec<String> = people
        .iter()
        .filter(|p| p.parent.borrow().upgrade().is_none())
        .map(|p| p.name.clone())
        .collect();
    let paths = people.iter().map(path).collect();
    let weak: Vec<Weak<Person>> = people.iter().map(Rc::downgrade).collect();
    drop(people);
    let freed = weak.iter().all(|w| w.upgrade().is_none());
    (roots, paths, freed)
}
