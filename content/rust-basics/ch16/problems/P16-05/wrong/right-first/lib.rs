use std::thread;

pub fn first_match(values: &[u32], target: u32) -> Option<usize> {
    let mid = values.len() / 2;
    let (left, right) = values.split_at(mid);
    thread::scope(|s| {
        let a = s.spawn(|| left.iter().position(|v| *v == target));
        let b = s.spawn(|| right.iter().position(|v| *v == target).map(|i| i + mid));
        let l = a.join().expect("the search doesn't panic");
        let r = b.join().expect("the search doesn't panic");
        r.or(l)
    })
}
