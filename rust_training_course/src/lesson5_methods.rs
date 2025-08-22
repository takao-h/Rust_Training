// ここに `User` 構造体と、その `impl` ブロックを実装してください。

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
        // ここを実装してください
        User { id, name, email, active: true }
    }

    // メソッド
    pub fn deactivate(&mut self) {
        // ここを実装してください
        self.active = false;
    }

    // メソッド
    pub fn full_name(&self) -> String {
        // ここを実装してください
        format!("{} ({})", self.name, self.email)
    }
}
