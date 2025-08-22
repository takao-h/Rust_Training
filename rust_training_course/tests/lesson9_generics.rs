use rust_training_course::lesson9_generics::*;

#[test]
fn test_largest_i32() {
    let number_list = vec![34, 50, 25, 100, 65];
    assert_eq!(largest(&number_list), 100);
}

#[test]
fn test_largest_char() {
    let char_list = vec!['y', 'm', 'a', 'q'];
    assert_eq!(largest(&char_list), 'y');
}

#[test]
fn test_largest_float() {
    let float_list = vec![1.1, 2.2, 0.5, 3.0];
    assert_eq!(largest(&float_list), 3.0);
}
