use std::cell::RefCell;
use std::rc::Rc;

pub fn broadcast(start: u32, updates: &[u32]) -> (u32, usize) {
    let counter = Rc::new(RefCell::new(start));
    let owners: Vec<Rc<RefCell<u32>>> = updates.iter().map(|_| Rc::clone(&counter)).collect();
    for (owner, by) in owners.iter().zip(updates) {
        let mut v = owner.borrow_mut();
        *v = v.saturating_add(*by);
    }
    let value = *counter.borrow();
    drop(owners);
    (value, Rc::strong_count(&counter))
}
