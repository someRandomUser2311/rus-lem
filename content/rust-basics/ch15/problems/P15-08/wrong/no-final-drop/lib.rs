use std::cell::RefCell;
use std::rc::Rc;

pub struct Ticket {
    pub name: String,
    pub log: Rc<RefCell<Vec<String>>>,
}

impl Drop for Ticket {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.name.clone());
    }
}

pub fn replacement(old: &str, new: &str) -> Vec<String> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut ticket = Ticket {
        name: old.to_string(),
        log: Rc::clone(&log),
    };
    ticket = Ticket {
        name: new.to_string(),
        log: Rc::clone(&log),
    };
    drop(ticket);
    let out = log.borrow().clone();
    out
}

pub fn swap_in(names: &[&str]) -> Vec<String> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut slot: Option<Ticket> = None;
    for name in names {
        slot = Some(Ticket {
            name: name.to_string(),
            log: Rc::clone(&log),
        });
    }
    std::mem::forget(slot);
    let out = log.borrow().clone();
    out
}
