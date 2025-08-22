use rust_training_course::lesson10_modules::add_one;

#[test]
fn test_add_one_from_module() {
    assert_eq!(add_one(5), 6);
}
