use std::cell::RefCell;

pub fn try_add(cell: &RefCell<u32>, by: u32) -> bool {
    match cell.try_borrow_mut() {
        Ok(mut v) => {
            *v = v.wrapping_add(by);
            true
        }
        Err(_) => false,
    }
}
