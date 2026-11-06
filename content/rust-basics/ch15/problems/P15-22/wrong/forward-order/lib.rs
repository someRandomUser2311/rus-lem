use std::cell::RefCell;
use std::rc::Rc;

pub struct Resource {
    pub name: String,
    pub log: Rc<RefCell<Vec<String>>>,
}

impl Drop for Resource {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.name.clone());
    }
}

pub fn cleanup(names: &[&str], early: usize) -> Vec<String> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut slots: Vec<Option<Box<Resource>>> = names
        .iter()
        .map(|n| {
            Some(Box::new(Resource {
                name: n.to_string(),
                log: Rc::clone(&log),
            }))
        })
        .collect();
    if let Some(slot) = slots.get_mut(early) {
        drop(slot.take());
    }
    for slot in slots.into_iter() {
        drop(slot);
    }
    let out = log.borrow().clone();
    out
}
