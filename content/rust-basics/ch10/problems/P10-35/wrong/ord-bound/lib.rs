pub fn merge<'a, T: Ord>(a: &'a [T], b: &'a [T]) -> Vec<&'a T> {
    let mut out = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            out.push(&a[i]);
            i += 1;
        } else {
            out.push(&b[j]);
            j += 1;
        }
    }
    while i < a.len() {
        out.push(&a[i]);
        i += 1;
    }
    while j < b.len() {
        out.push(&b[j]);
        j += 1;
    }
    out
}
