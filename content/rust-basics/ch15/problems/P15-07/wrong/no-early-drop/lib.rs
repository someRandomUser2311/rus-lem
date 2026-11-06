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

fn ticket(name: &str, log: &Rc<RefCell<Vec<String>>>) -> Ticket {
    Ticket {
        name: name.to_string(),
        log: Rc::clone(log),
    }
}

pub fn cleanup_order() -> Vec<String> {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _outer = ticket("outer", &log);
        {
            let first = ticket("first", &log);
            let _second = ticket("second", &log);
        }
    }
    log.borrow().clone()
}

pub fn repeated_order(n: u32) -> Vec<String> {
    let mut out = Vec::new();
    for _ in 0..n {
        out.extend(cleanup_order());
    }
    out
}
