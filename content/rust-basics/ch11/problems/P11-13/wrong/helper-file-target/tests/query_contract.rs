mod common;

use test_lab::matches;

#[test]
fn finds_a_word() {
    assert!(matches("the rust book", "rust"));
}

#[test]
fn needs_every_query_word() {
    assert!(matches("the rust book", "rust book"), "both words are there");
    assert!(!matches("the rust book", "rust python"), "python is missing");
}

#[test]
fn ignores_capitals_on_both_sides() {
    let title = common::unicode_title();
    assert!(matches(&title, "rust"), "the title says Rust");
    assert!(matches("the rust book", "RUST"), "the query is lowercased too");
}

#[test]
fn an_empty_query_matches_everything() {
    assert!(matches("anything", ""));
    assert!(matches("anything", " \t "));
}
