// ここに `User` 構造体と `find_user_by_id` 関数を実装してください。

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

pub fn find_user_by_id(id: u32) -> Option<User> {
    // ここにユーザーを検索するロジックを実装してください
    // 例: ユーザーのリストを作成し、idで検索します。
    // 見つかった場合は Some(User) を、見つからない場合は None を返します。
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            active: true,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            active: false,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
            active: true,
        },
    ];
    let found_user_ref: Option<&User> = users.iter().find(|user| user.id == id);
    found_user_ref.cloned()
}
