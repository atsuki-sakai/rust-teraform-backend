# src/domain/entities/todo.rs 解説

このファイルは、アプリケーションの **中心となるデータ構造（エンティティ）** である「Todo」を定義しています。

## 構造体定義
```rust
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```
- `struct Todo`: Todoアイテムの設計図です。
- `derive(...)`: 自動的に便利な機能を追加します。
    - `Debug`: `println!` で中身を表示できるようにします。
    - `Clone`: 自身のコピーを作成できるようにします。
    - `Serialize`, `Deserialize`: JSON と相互変換できるようにします（`serde`）。
    - `ToSchema`: Swagger (OpenAPI) ドキュメント用のスキーマを自動生成します（`utoipa`）。
    - `sqlx::FromRow`: データベースの検索結果（行）をこの構造体に自動変換します。

各フィールド：
- `id`: 一意な識別子 (UUID)。
- `user_id`: 作成者のユーザーID。
- `description`: 詳細説明。`Option<String>` なので `None`（データなし）も許容します。
- `created_at`, `updated_at`: 作成日時と更新日時。

## メソッド実装 (Methods)
```rust
impl Todo {
    pub fn new(...) -> Self { ... }
    pub fn update(&mut self, ...) { ... }
}
```
- `new`: 新しい Todo を作成するためのコンストラクタ的な関数です。IDの生成や現在時刻の設定を行います。
- `update`: Todo の内容を更新します。引数が `Option` なので、更新したい項目だけを指定できます。
