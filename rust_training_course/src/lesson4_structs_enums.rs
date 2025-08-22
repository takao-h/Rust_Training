// ここに `User` 構造体と `find_user_by_id` 関数を実装してください。

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

pub fn find_user_by_id(id: u32) -> Option<User> {
    // 仮のデータ
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string(), active: true },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string(), active: false },
    ];

    // ここにユーザーを検索するロジックを実装してください
    // 見つかった場合は Some(User) を、見つからない場合は None を返します
    None
}
