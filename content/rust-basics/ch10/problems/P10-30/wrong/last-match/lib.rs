pub fn borrowed_lookup<'a, T: PartialEq>(items: &'a [T], wanted: &T) -> Option<&'a T> {
    let mut found = None;
    for item in items {
        if item == wanted {
            found = Some(item);
        }
    }
    found
}
