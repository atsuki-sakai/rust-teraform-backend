# src/domain/entities/todo.rs 解説

このファイルは、アプリケーションの **中心となるデータ構造（エンティティ）** である「Todo」を定義しています。

## 構造体定義
```rust
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

// Newtype Pattern: 型安全性とバリデーションの強化
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(transparent)]
pub struct TodoId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(transparent)]
pub struct TodoTitle(String);

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Todo {
    pub id: TodoId,
    pub user_id: Uuid,
    pub title: TodoTitle,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Newtype Pattern (ニュータイプパターン)
Rust特有の強力なデザインパターンです。プリミティブ型（`String`や`Uuid`）を独自の型でラップすることで、以下のメリットがあります：

1.  **型安全性**: `TodoId` と `UserId`（どちらも中身は UUID）を取り違えるミスをコンパイル時に防げます。
2.  **バリデーションの集約**: `TodoTitle` などの型自体に作成メソッド（`new`）を持たせ、そこで「空文字禁止」「文字数制限」などのルールを強制します。これにより、不正なデータがドメイン層に存在することを防ぎます。
3.  **`#[sqlx(transparent)]`**: データベースとのやり取りでは、ラップされている中身の型（`Uuid`や`String`）として透過的に扱われます。

## 構造体詳細
- `struct Todo`: Todoアイテムの設計図です。
- `derive(...)`: 自動的に便利な機能を追加します。
    - `Type` (sqlx): データベースの型とマッピングします。
    - `ToSchema`: Swagger (OpenAPI) ドキュメント用のスキーマを自動生成します。
    - `sqlx::FromRow`: データベースの検索結果（行）をこの構造体に自動変換します。

各フィールド：
- `id`: 一意な識別子 (`TodoId`)。
- `user_id`: 作成者のユーザーID。
- `title`: タイトル (`TodoTitle`)。必ず有効な値であることが保証されます。
- `description`: 詳細説明。`Option<String>` なので `None`（データなし）も許容します。

## メソッド実装 (Methods)
```rust
impl TodoTitle {
    pub fn new(title: String) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        // ... バリデーション
        Ok(Self(title))
    }
}

impl Todo {
    pub fn new(user_id: Uuid, title: TodoTitle, description: Option<String>) -> Self { ... }
    pub fn update(&mut self, title: Option<TodoTitle>, ...) { ... }
}
```
- `TodoTitle::new`: タイトルのルール（空文字禁止など）をチェックし、成功した場合のみ `TodoTitle` を返します。
- `Todo::new`: 必ず有効な `TodoTitle` を受け取るため、作成される `Todo` は常に整合性が保たれます。
