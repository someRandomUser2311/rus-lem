pub fn read_via_ptr(x: &i32) -> i32 {
    let p = x as *const i32;
    // SAFETY: p comes from a live shared reference, so it is valid and aligned.
    unsafe { *p }
}

pub fn bump_via_ptr(x: &mut i32, by: i32) {
    let p = x as *mut i32;
    // SAFETY: p comes from a live exclusive reference, so nothing else accesses the value.
    unsafe {
        *p = (*p).wrapping_add(by);
    }
}
