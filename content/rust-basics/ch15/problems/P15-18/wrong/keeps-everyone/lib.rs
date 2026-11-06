use std::rc::{Rc, Weak};

pub fn live_names(names: &[&str], keep: &[bool]) -> Vec<String> {
    let owners: Vec<Rc<String>> = names.iter().map(|n| Rc::new(n.to_string())).collect();
    let weak: Vec<Weak<String>> = owners.iter().map(Rc::downgrade).collect();
    let kept: Vec<Rc<String>> = owners
        .into_iter()
        .zip(keep)
        .filter(|(_, _k)| true)
        .map(|(o, _)| o)
        .collect();
    let live = weak
        .iter()
        .filter_map(|w| w.upgrade())
        .map(|s| s.to_string())
        .collect();
    drop(kept);
    live
}
