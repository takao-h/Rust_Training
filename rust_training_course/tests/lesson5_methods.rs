use rust_training_course::lesson5_methods::*;

#[test]
fn test_user_new() {
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert!(user.active);
}

#[test]
fn test_user_deactivate() {
    let mut user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    user.deactivate();
    assert!(!user.active);
}

#[test]
fn test_user_full_name() {
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    assert_eq!(user.full_name(), "Alice (alice@example.com)");
}
