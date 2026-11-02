#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Library {
        let mut library = Library::new();
        library.add_book("Dune", "Frank Herbert", 1965);
        library.add_book("Emma", "Jane Austen", 1815);
        library.add_book("Ulysses", "James Joyce", 1922);
        library.add_book("Persuasion", "Jane Austen", 1817);
        library
    }

    #[test]
    fn index_of_finds_positions_by_id() {
        let library = sample();
        assert_eq!(library.index_of(1), Some(0), "the first book");
        assert_eq!(library.index_of(4), Some(3), "the last book");
        assert_eq!(library.index_of(9), None, "no book has id 9");
    }

    #[test]
    fn standard_members_borrow_three_and_premium_six() -> Result<(), LibraryError> {
        let mut library = sample();
        library.add_member(7, "Ada", false)?;
        library.add_member(8, "Bo", true)?;
        for id in 1..=3 {
            library.checkout(id, 7, 1)?;
        }
        assert_eq!(
            library.checkout(4, 7, 1),
            Err(LibraryError::LimitReached(7)),
            "a fourth book is over a standard member's limit"
        );
        match library.member(8) {
            Some(member) => assert_eq!(member.limit(), 6, "premium members may borrow six"),
            None => panic!("member 8 was added"),
        }
        Ok(())
    }

    #[test]
    fn returning_a_book_frees_a_place() -> Result<(), LibraryError> {
        let mut library = sample();
        library.add_member(7, "Ada", false)?;
        for id in 1..=3 {
            library.checkout(id, 7, 1)?;
        }
        library.return_book(2, 5)?;
        library.checkout(4, 7, 5)?;
        assert_eq!(
            library.count_by_status(),
            (1, 3, 0),
            "one on the shelf, three out"
        );
        Ok(())
    }

    #[test]
    fn members_are_sorted_by_id() -> Result<(), LibraryError> {
        let mut library = Library::new();
        library.add_member(30, "Cy", false)?;
        library.add_member(10, "Ann", false)?;
        library.add_member(20, "Bo", true)?;
        let mut ids = Vec::new();
        for member in library.members_sorted() {
            ids.push(member.id);
        }
        assert_eq!(ids, vec![10, 20, 30], "ascending ids");
        Ok(())
    }
}
