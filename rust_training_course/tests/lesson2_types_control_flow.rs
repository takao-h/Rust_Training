use rust_training_course::lesson2_types_control_flow::*;

#[test]
fn test_check_number_positive() {
    assert_eq!(check_number(5), "Positive");
}

#[test]
fn test_check_number_negative() {
    assert_eq!(check_number(-10), "Negative");
}

#[test]
fn test_check_number_zero() {
    assert_eq!(check_number(0), "Zero");
}
