use test_lab::{Item, Shelf};

fn item(id: u32, label: &str) -> Item {
    Item { id, label: String::from(label) }
}

fn label_of(shelf: &Shelf<Item>, id: u32) -> String {
    match shelf.find(id) {
        Some(item) => item.label.clone(),
        None => String::from("<none>"),
    }
}

#[test]
fn finds_the_only_item() {
    let mut shelf = Shelf::new();
    shelf.add(item(1, "lamp"));
    assert_eq!(label_of(&shelf, 1), "lamp");
}

#[test]
fn the_first_duplicate_wins() {
    let mut shelf = Shelf::new();
    shelf.add(item(2, "old"));
    shelf.add(item(2, "new"));
    assert_eq!(label_of(&shelf, 2), "old");
}

#[test]
fn a_missing_key_is_none() {
    let mut shelf = Shelf::new();
    shelf.add(item(1, "lamp"));
    assert!(shelf.find(9).is_none(), "nothing has id 9");
}

#[test]
fn len_counts_every_add() {
    let mut shelf = Shelf::new();
    assert_eq!(shelf.len(), 0, "a new shelf is empty");
    shelf.add(item(1, "a"));
    shelf.add(item(2, "b"));
    shelf.add(item(3, "c"));
    assert_eq!(shelf.len(), 3, "three adds");
    assert_eq!(label_of(&shelf, 1), "a", "earlier items stay on the shelf");
}

#[test]
fn any_type_can_be_stored() {
    let mut shelf: Shelf<String> = Shelf::new();
    shelf.add(String::from("no key needed"));
    shelf.add(String::from("東京"));
    assert_eq!(shelf.len(), 2);
}
