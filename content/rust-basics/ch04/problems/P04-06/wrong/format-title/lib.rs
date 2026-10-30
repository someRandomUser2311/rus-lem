pub fn second_edition(title: String, year: u32) -> ((String, u32), (String, u32)) {
    let second = format!("{title} (2nd ed.)");
    ((title, year), (second, year + 1))
}
