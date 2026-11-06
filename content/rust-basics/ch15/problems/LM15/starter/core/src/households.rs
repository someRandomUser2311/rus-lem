use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::member::Member;

pub struct Household {
    pub name: String,
    balance_cents: u32,
    members: Vec<Weak<RefCell<Member>>>,
}

pub type HouseholdRef = Rc<RefCell<Household>>;

pub fn new_household(name: &str) -> HouseholdRef {
    Rc::new(RefCell::new(Household {
        name: String::new(),
        balance_cents: 0,
        members: Vec::new(),
    }))
}

pub fn balance(h: &HouseholdRef) -> u32 {
    0
}

pub fn charge(h: &HouseholdRef, cents: u32) {}

pub fn join(h: &HouseholdRef, member: &Rc<RefCell<Member>>) -> bool {
    false
}

pub fn member_names(h: &HouseholdRef) -> Vec<String> {
    Vec::new()
}

pub fn prune(h: &HouseholdRef) -> usize {
    0
}

pub fn charge_late(h: &HouseholdRef, due_day: u32, return_day: u32) -> u32 {
    0
}
