use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

pub fn node(name: &str) -> Rc<Node> {
    Rc::new(Node {
        name: name.to_string(),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    })
}

pub fn link(parent: &Rc<Node>, child: &Rc<Node>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

pub fn weak_tree(name: &str) -> (bool, usize, bool, usize) {
    let child = node(name);
    let root = node("root");
    link(&root, &child);
    let alive = child.parent.borrow().upgrade().is_some();
    let before = Rc::strong_count(&child) - 1;
    drop(root);
    let gone = child.parent.borrow().upgrade().is_none();
    (alive, before, gone, Rc::strong_count(&child))
}
