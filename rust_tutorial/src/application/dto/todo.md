# src/application/dto/todo_dto.rs 解説

DTO (Data Transfer Object) は、API の **リクエスト（入力）とレスポンス（出力）の形式** を定義するファイルです。
データベースの構造（Entity）とは切り離して、外に見せたい形を定義します。

## リクエスト用 DTO
```rust
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}
```

### 🦀 詳しい解説: `#[derive(...)]` とは？

コードの上についている `#[derive(Debug, Deserialize, ToSchema)]` は、Rust の強力な機能の一つで、**「この構造体に便利な機能を自動的に追加して！」** というコンパイラへの命令です。

1.  **`Debug`**:
    - 開発中に `println!("{:?}", request)` などで、データの**中身をログに表示**できるようにします。これがないと、エラー時に中身を確認するのが大変になります。

2.  **`Deserialize`** (デシリアライズ):
    - `serde` (サード) というライブラリの機能です。
    - クライアントから送られてきた **JSONデータ（文字列）を、この Rust の構造体に自動変換** します。
    - 例: `{"title": "買い物"}` という JSON が来たら、自動で `title` フィールドに代入してくれます。

3.  **`ToSchema`**:
    - `utoipa` というライブラリの機能です。
    - この構造体の形を読み取って、**API 仕様書 (Swagger UI) を自動生成** します。
    - 手動でドキュメントを書かなくても、コードを書くだけで最新のドキュメントが出来上がります。
```
- `CreateTodoRequest`: Todo 作成時にクライアントから送られてくるデータの形です。
- `UpdateTodoRequest`: Todo 更新時に送られてくるデータの形です。すべてのフィールドが `Option` なので、変更したい項目だけ送信できます。

## レスポンス用 DTO
```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoResponse {
```
- **`Serialize`** (シリアライズ):
    - `Deserialize` の逆です。**「Rust の構造体 → JSONデータ」** への変換を可能にします。
    - API がレスポンスを返す時に、Rust のデータをきれいな JSON にしてクライアントに送るために必須です。 ... }

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self { ... }
}
```
- `TodoResponse`: API がクライアントに返すデータの形です。
- `impl From<Todo> for TodoResponse`: 内部データの `Todo` (Entity) から、表示用の `TodoResponse` に変換するロジックです。これにより `todo.into()` と書くだけで変換できるようになります。

## ページネーション
```rust
pub struct TodoListResponse {
    pub todos: Vec<TodoResponse>,
    pub total: i64,
    ...
}

pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
```
- `TodoListResponse`: 一覧取得時のレスポンス。リスト本体 (`todos`) に加えて、ページ番号や総件数などのメタデータも含みます。
- `PaginationQuery`: URL のクエリパラメータ（例: `?page=2&per_page=10`）を受け取るための構造体です。未指定時のデフォルト値（1ページ目、20件など）を処理するロジックも含まれています。
