use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::fees::{days_late, late_fee_cents};
use crate::member::Member;

pub struct Household {
    pub name: String,
    balance_cents: u32,
    members: Vec<Weak<RefCell<Member>>>,
}

pub type HouseholdRef = Rc<RefCell<Household>>;

pub fn new_household(name: &str) -> HouseholdRef {
    Rc::new(RefCell::new(Household {
        name: name.to_string(),
        balance_cents: 0,
        members: Vec::new(),
    }))
}

pub fn balance(h: &HouseholdRef) -> u32 {
    h.borrow().balance_cents
}

pub fn charge(h: &HouseholdRef, cents: u32) {
    let mut hh = h.borrow_mut();
    hh.balance_cents = hh.balance_cents.saturating_add(cents);
}

pub fn join(h: &HouseholdRef, member: &Rc<RefCell<Member>>) -> bool {
    let mut hh = h.borrow_mut();
    if hh
        .members
        .iter()
        .any(|w| w.upgrade().is_some_and(|m| Rc::ptr_eq(&m, member)))
    {
        return false;
    }
    hh.members.push(Rc::downgrade(member));
    true
}

pub fn member_names(h: &HouseholdRef) -> Vec<String> {
    let mut names: Vec<String> = h
        .borrow()
        .members
        .iter()
        .filter_map(|w| w.upgrade())
        .map(|m| m.borrow().name.clone())
        .collect();
    names.sort();
    names
}

pub fn prune(h: &HouseholdRef) -> usize {
    let mut hh = h.borrow_mut();
    let before = hh.members.len();
    hh.members.retain(|w| w.upgrade().is_some());
    before - hh.members.len()
}

pub fn charge_late(h: &HouseholdRef, due_day: u32, return_day: u32) -> u32 {
    let fee = late_fee_cents(days_late(due_day, return_day));
    charge(h, fee);
    fee
}
