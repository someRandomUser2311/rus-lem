pub fn append_twice(text: &mut String) -> () {
    let first = &mut *text;
    first.push('?');
    let second = &mut *text;
    second.push('!');
}
