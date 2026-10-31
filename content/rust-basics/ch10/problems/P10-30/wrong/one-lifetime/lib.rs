pub fn borrowed_lookup<'a, T: PartialEq>(items: &'a [T], wanted: &'a T) -> Option<&'a T> {
    for item in items {
        if item == wanted {
            return Some(item);
        }
    }
    None
}
