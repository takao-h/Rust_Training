use rust_training_course::lesson1_basics::*;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add_negative() {
    assert_eq!(add(-1, -1), -2);
}
