pub fn formatter(prefix: String) -> impl Fn(&str) -> String {
    move |text| format!("{text}")
}
