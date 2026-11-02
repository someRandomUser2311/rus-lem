use test_lab::{clamp, normalize, parse_count, valid};

#[test]
fn clamp_from_outside() {
    assert_eq!(clamp(7), 7);
}

#[test]
fn normalize_from_outside() {
    assert_eq!(normalize("\tThe\u{3000}Rust  Book "), "The Rust Book");
}

#[test]
fn parse_count_from_outside() -> Result<(), String> {
    assert_eq!(parse_count(" 7 ")?, 7);
    assert_eq!(parse_count("seven"), Err(String::from("invalid")));
    Ok(())
}

#[test]
fn valid_from_outside() {
    assert_eq!(valid(1), 1);
    assert_eq!(valid(100), 100);
}

#[test]
#[should_panic(expected = "at most one hundred")]
fn valid_panics_from_outside() {
    valid(101);
}
