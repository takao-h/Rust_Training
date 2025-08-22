use rust_training_course::lesson7_error_handling::*;

#[test]
fn test_parse_and_add_success() {
    assert_eq!(parse_and_add("10", "20").unwrap(), 30);
}

#[test]
fn test_parse_and_add_failure() {
    let err = parse_and_add("abc", "20").unwrap_err();
    assert_eq!(err.to_string(), "invalid digit found in string");

    let err = parse_and_add("10", "xyz").unwrap_err();
    assert_eq!(err.to_string(), "invalid digit found in string");
}
