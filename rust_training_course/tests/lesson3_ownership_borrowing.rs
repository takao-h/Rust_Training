use rust_training_course::lesson3_ownership_borrowing::*;

#[test]
fn test_take_ownership() {
    let s = String::from("hello");
    take_ownership(s);
    // ここで s を使おうとするとコンパイルエラーになることを確認してください
    // 例: println!("{}", s);
}

#[test]
fn test_borrow_string() {
    let s = String::from("hello");
    let len = borrow_string(&s);
    assert_eq!(len, 5);
    // 借用なので、s は引き続き使えます
    println!("Original string: {}", s);
}

#[test]
fn test_change_string() {
    let mut s = String::from("hello");
    change_string(&mut s);
    assert_eq!(s, "hello, world!");
}
