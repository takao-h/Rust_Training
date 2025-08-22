// ここに `take_ownership`, `borrow_string`, `change_string` 関数を実装してください。

// take_ownership: Stringを受け取り、所有権をムーブさせる関数
pub fn take_ownership(s: String) {
    // 何もしなくても所有権はムーブします
}

// borrow_string: &strを受け取り、借用する関数
pub fn borrow_string(s: &str) -> usize {
    // 文字列の長さを返すなど、参照を使って何かをします
    s.len()
}

// change_string: &mut Stringを受け取り、文字列を変更する関数
pub fn change_string(s: &mut String) {
    s.push_str(", world!");
}
