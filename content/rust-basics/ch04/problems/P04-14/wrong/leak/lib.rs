pub fn make_label(name: String) -> &'static String {
    Box::leak(Box::new(name))
}
