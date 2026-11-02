mod common;

use test_lab::{first_word, longest_word, normalize, View};

#[test]
fn finds_the_first_word() {
    assert_eq!(first_word("hello world"), "hello");
}

#[test]
fn the_first_word_of_the_fixture() {
    let text = common::unicode_fixture();
    assert_eq!(first_word(&text), "éé", "U+3000 is whitespace");
}

#[test]
fn no_words_give_empty_text() {
    let blank = String::from(" \t ");
    assert_eq!(first_word(&blank), "");
    assert_eq!(longest_word(&blank), "");
}

#[test]
fn the_longest_word_counts_characters() {
    let text = common::unicode_fixture();
    assert_eq!(longest_word(&text), "abc", "3 characters beat 2, though éé has more bytes");
}

#[test]
fn the_first_of_equal_words_wins() {
    let text = String::from("ab cd");
    assert_eq!(longest_word(&text), "ab");
}

#[test]
fn normalize_collapses_inner_runs() {
    assert_eq!(normalize("a  \t b\u{3000}\u{3000}c"), "a b c");
    assert_eq!(normalize(&common::unicode_fixture()), "éé abc");
}

#[test]
fn a_view_shows_its_text() {
    let text = String::from("  first second");
    let view = View::new(&text);
    assert_eq!(view.first(), "first");
    assert_eq!(view.original(), "  first second");
}
