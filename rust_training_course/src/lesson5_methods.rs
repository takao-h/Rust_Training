// ここに `User` 構造体と、その `impl` ブロックを実装してください。

use std::fmt::format;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

impl User {
    // 関連関数 (コンストラクタのようなもの)
    pub fn new(id: u32, name: String, email: String) -> User {
        User { id, name, email, active: true }
    }

    // メソッド
    pub fn deactivate(&mut self) {
        self.active = false
    }

    // メソッド
    pub fn full_name(&self) -> String {
        format!("{} ({})", self.name, self.email)
    }
}