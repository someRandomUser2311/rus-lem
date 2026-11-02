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
fn finds_each_item_by_its_own_key() {
    let mut shelf = Shelf::new();
    shelf.add(item(3, "a"));
    shelf.add(item(7, "b"));
    shelf.add(item(5, "c"));
    assert_eq!(label_of(&shelf, 3), "a", "the first item");
    assert_eq!(label_of(&shelf, 7), "b", "the middle item");
    assert_eq!(label_of(&shelf, 5), "c", "the last item");
}

#[test]
fn a_missing_key_is_none() {
    let mut shelf = Shelf::new();
    shelf.add(item(1, "lamp"));
    assert!(shelf.find(9).is_none(), "nothing has id 9");
}

#[test]
fn extreme_ids_are_ordinary_keys() {
    let mut shelf = Shelf::new();
    shelf.add(item(0, "zero"));
    shelf.add(item(u32::MAX, "max"));
    assert_eq!(label_of(&shelf, 0), "zero", "id 0 is a key like any other");
    assert_eq!(label_of(&shelf, u32::MAX), "max", "so is the largest id");
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
