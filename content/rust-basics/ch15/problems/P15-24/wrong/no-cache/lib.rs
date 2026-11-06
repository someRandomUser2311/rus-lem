use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct Interner {
    cache: HashMap<String, Weak<String>>,
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            cache: HashMap::new(),
        }
    }

    pub fn intern(&mut self, text: &str) -> Rc<String> {
        let rc = Rc::new(text.to_string());
        self.cache.insert(text.to_string(), Rc::downgrade(&rc));
        rc
    }

    pub fn live(&self) -> usize {
        self.cache.values().filter(|w| w.upgrade().is_some()).count()
    }

    pub fn purge(&mut self) {
        self.cache.retain(|_, w| w.upgrade().is_some());
    }
}

pub fn identities(texts: &[&str]) -> (Vec<usize>, usize) {
    let mut interner = Interner::new();
    let handles: Vec<Rc<String>> = texts.iter().map(|t| interner.intern(t)).collect();
    let firsts = handles
        .iter()
        .map(|h| handles.iter().position(|o| Rc::ptr_eq(o, h)).unwrap_or(0))
        .collect();
    (firsts, interner.live())
}
