mod common;

use test_lab::normalize;

#[test]
fn joins_words_with_one_space() {
    assert_eq!(normalize("a b"), "a b");
}

#[test]
fn splits_on_any_unicode_whitespace() {
    let title = common::unicode_title();
    assert_eq!(normalize(&title), "The Rust Book", "tabs and U+3000 separate words too");
}

#[test]
fn collapses_runs_of_spaces() {
    assert_eq!(normalize("  a   b  "), "a b");
}

#[test]
fn whitespace_only_text_is_empty() {
    assert_eq!(normalize(" \t\u{3000} "), "");
}
