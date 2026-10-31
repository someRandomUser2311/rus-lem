pub fn borrowed_lookup<'a, T: PartialEq + std::fmt::Debug>(items: &'a [T], wanted: &T) -> Option<&'a T> {
    for item in items {
        if item == wanted {
            return Some(item);
        }
    }
    None
}
