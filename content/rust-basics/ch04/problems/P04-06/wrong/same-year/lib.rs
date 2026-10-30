pub fn second_edition(title: String, year: u32) -> ((String, u32), (String, u32)) {
    let mut second = title.clone();
    second.push_str(" (2nd ed.)");
    ((title, year), (second, year))
}
