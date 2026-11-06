use std::cell::RefCell;
use std::rc::Rc;

pub fn transfer(from: &Rc<RefCell<u32>>, to: &Rc<RefCell<u32>>, amount: u32) -> Result<(), &'static str> {
    if Rc::ptr_eq(from, to) {
        return Ok(());
    }
    if *from.borrow() < amount {
        return Err("funds");
    }
    let mut a = from.try_borrow_mut().map_err(|_| "busy")?;
    let mut b = to.try_borrow_mut().map_err(|_| "busy")?;
    let new_b = b.checked_add(amount).ok_or("overflow")?;
    *a -= amount;
    *b = new_b;
    Ok(())
}
