# src/presentation/routes/todo_routes.rs 解説

このファイルは、Todo 機能に関する **API の URL 設計図（ルーティング）** です。
どの URL にどんなリクエストが来たら、どのハンドラ（処理担当）に渡すかを定義します。

## ルーティング定義
```rust
pub fn todo_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(todo_handlers::list_todos))
        .route("/", post(todo_handlers::create_todo))
        .route("/{id}", get(todo_handlers::get_todo))
        .route("/{id}", put(todo_handlers::update_todo))
        .route("/{id}", delete(todo_handlers::delete_todo))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}
```
この定義は `main.rs` で `/api/v1/todos` の下に組み込まれるため、実際の URL は以下のようになります。

- **GET** `/api/v1/todos/`: 一覧取得 (`list_todos`)
- **POST** `/api/v1/todos/`: 新規作成 (`create_todo`)
- **GET** `/api/v1/todos/{id}`: 詳細取得 (`get_todo`)
- **PUT** `/api/v1/todos/{id}`: 更新 (`update_todo`)
- **DELETE** `/api/v1/todos/{id}`: 削除 (`delete_todo`)

## ポイント
- `.layer(...)`: 最後に `auth_middleware`（認証ミドルウェア）を適用しています。
    - これにより、ここにある **すべてのルートはログインが必要** になります。
    - ログインしていないユーザーがアクセスすると、ハンドラに到達する前にエラー（401 Unauthorized）が返されます。
