use std::i32;

// ここに `check_number` 関数を実装してください。
// i32型の引数を取り、その値に応じて "Positive", "Negative", "Zero" のいずれかのStringを返します。
pub fn check_number(a: i32) -> String {
    match a {
        0 => "Zero".to_string(),
        1..=i32::MAX => "Positive".to_string(),
        i32::MIN..=-1 => "Negative".to_string(),
    }
}
