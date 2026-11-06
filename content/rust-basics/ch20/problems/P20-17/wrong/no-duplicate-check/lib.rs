use std::collections::HashSet;

fn check(len: usize, i: usize) -> Result<(), String> {
    if i < len {
        Ok(())
    } else {
        Err(format!("index {i} out of bounds for length {len}"))
    }
}

pub fn scatter(values: &mut [i32], updates: &[(usize, i32)]) -> Result<usize, String> {
    let len = values.len();
    let mut seen = HashSet::new();
    for &(i, _) in updates {
        check(len, i)?;
        seen.insert(i);
    }
    let base = values.as_mut_ptr();
    // SAFETY: every index was checked to be in bounds and distinct.
    unsafe {
        for &(i, v) in updates {
            *base.add(i) = v;
        }
    }
    Ok(updates.len())
}

pub fn gather(values: &[i32], indices: &[usize]) -> Result<Vec<i32>, String> {
    for &i in indices {
        check(values.len(), i)?;
    }
    let base = values.as_ptr();
    // SAFETY: every index was checked to be in bounds; reads don't conflict.
    unsafe { Ok(indices.iter().map(|&i| *base.add(i)).collect()) }
}
