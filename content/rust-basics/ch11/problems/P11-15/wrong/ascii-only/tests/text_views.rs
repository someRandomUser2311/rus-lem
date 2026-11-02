mod common;

use test_lab::{first_word, longest_word, normalize, same_location, View};

#[test]
fn finds_the_first_word() {
    assert_eq!(first_word("hello world"), "hello");
}

#[test]
fn the_first_word_is_borrowed_from_the_input() {
    let text = String::from(" ab cdef ");
    let word = first_word(&text);
    assert_eq!(word, "ab", "the first word");
    assert!(same_location(word, &text[1..3]), "the word is a slice of the input, not a copy");
}

#[test]
fn no_words_give_the_empty_slice_at_the_start() {
    let blank = String::from(" \t ");
    assert_eq!(first_word(&blank), "");
    assert!(same_location(first_word(&blank), &blank[0..0]), "first_word points at the start");
    assert!(same_location(longest_word(&blank), &blank[0..0]), "longest_word points at the start");
}

#[test]
fn the_longest_word_counts_characters() {
    let text = String::from("ab cdef");
    let word = longest_word(&text);
    assert_eq!(word, "cdef", "4 characters beat 2");
    assert!(same_location(word, &text[3..7]), "the slice of the input");
}

#[test]
fn the_first_of_equal_words_wins() {
    let text = String::from("ab cd");
    assert!(same_location(longest_word(&text), &text[0..2]), "ab comes first");
}

#[test]
fn normalize_collapses_inner_runs() {
    assert_eq!(normalize("a  \t b\u{3000}\u{3000}c"), "a b c");
    assert_eq!(common::unicode_fixture().is_empty(), false);
}

#[test]
fn a_view_borrows_its_text() {
    let text = String::from("  first second");
    let view = View::new(&text);
    assert_eq!(view.first(), "first");
    assert!(same_location(view.first(), &text[2..7]), "first() is a slice of the text");
    assert!(same_location(view.original(), &text), "original() is the whole text");
}
