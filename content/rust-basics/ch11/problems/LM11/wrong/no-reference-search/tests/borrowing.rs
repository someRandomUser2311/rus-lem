use library::{Library, LibraryError};

fn stocked() -> Library {
    let mut library = Library::new();
    library.add_book(
        "The  Rust\tProgramming\u{3000}Language",
        "Steve Klabnik",
        2019,
    );
    library.add_book("Dune", "Frank Herbert", 1965);
    library
}

#[test]
fn a_complete_borrowing_cycle() -> Result<(), LibraryError> {
    let mut library = stocked();
    library.add_member(1, "Ada", false)?;
    let due = library.checkout(2, 1, 10)?;
    assert_eq!(due, 24, "a standard loan is due 14 days later");
    let fee = library.return_book(2, 32)?;
    assert_eq!(fee, 225, "8 days late: 7 × 25 + 1 × 50 cents");
    assert_eq!(library.count_by_status(), (2, 0, 0), "both books are back");
    Ok(())
}

#[test]
fn titles_are_normalized_when_added() {
    let library = stocked();
    match library.book(1) {
        Some(book) => assert_eq!(book.title, "The Rust Programming Language"),
        None => panic!("book 1 was added"),
    }
}

#[test]
fn title_search_ignores_case() {
    let library = stocked();
    assert_eq!(library.search_titles("rust programming"), vec![1]);
}

#[test]
fn find_needs_every_word_in_title_or_author() {
    let library = stocked();
    let found = library.find("klabnik rust");
    assert_eq!(found.len(), 1, "one word in the author, one in the title");
    assert_eq!(found[0].id, 1);
    assert_eq!(
        library.find("rust herbert").len(),
        0,
        "no book has both words"
    );
}
