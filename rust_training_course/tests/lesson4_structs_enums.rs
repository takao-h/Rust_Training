use rust_training_course::lesson4_structs_enums::*;

#[test]
fn test_find_user_by_existing_id() {
    let user = find_user_by_id(1);
    assert!(user.is_some());
    let found_user = user.unwrap();
    assert_eq!(found_user.id, 1);
    assert_eq!(found_user.name, "Alice");
}

#[test]
fn test_find_user_by_non_existing_id() {
    let user = find_user_by_id(99);
    assert!(user.is_none());
}
