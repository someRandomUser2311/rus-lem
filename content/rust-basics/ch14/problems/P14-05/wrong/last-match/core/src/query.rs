use catalog_models::Record;

/// The first record whose name is exactly `name`.
///
/// # Examples
///
/// ```
/// use catalog_core::{first_named, Record};
///
/// let records = vec![
///     Record { id: 1, name: String::from("Ada") },
///     Record { id: 2, name: String::from("Bo") },
///     Record { id: 3, name: String::from("Bo") },
/// ];
/// assert_eq!(first_named(&records, "Bo").map(|r| r.id), Some(2));
/// assert!(first_named(&records, "bo").is_none());
/// ```
pub fn first_named<'a>(records: &'a [Record], name: &str) -> Option<&'a Record> {
    records.iter().rev().find(|r| r.name == name)
}

/// One line per record: `id: name`.
///
/// # Examples
///
/// ```
/// use catalog_core::{render, Record};
///
/// let records = vec![Record { id: 7, name: String::from("Cy") }];
/// assert_eq!(render(&records), "7: Cy\n");
/// ```
pub fn render(records: &[Record]) -> String {
    records
        .iter()
        .map(|r| format!("{}: {}\n", r.id, r.name))
        .collect()
}
