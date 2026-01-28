# src/application/services/todo_service.rs 解説

このファイルは、アプリケーションの **ビジネスロジック（実際の仕事）** を担当する「サービスクラス」です。
ユーザーからのリクエストを受け取り、リポジトリを使ってデータベースを操作し、結果を加工して返します。

## 構造体と依存性注入
```rust
pub struct TodoService {
    todo_repository: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(todo_repository: Arc<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }
}
```
- `todo_repository`: このサービスが仕事をするために必要な「データベース係」です。
- `Arc<dyn TodoRepository>`: ここがポイントです。具体的な `PostgresTodoRepository` ではなく、抽象的な `TodoRepository` トレイト（インターフェース）に依存しています。
    - これにより、将来データベースが変わっても、このロジックを変える必要がありません（依存性逆転の原則）。

## メソッド実装
例えば `create` メソッドを見てみましょう。
```rust
pub async fn create(&self, user_id: Uuid, request: CreateTodoRequest) -> AppResult<TodoResponse> {
    // 1. バリデーションとエンティティの作成
    let title = TodoTitle::new(request.title).map_err(AppError::Validation)?;
    let todo = Todo::new(user_id, title, request.description);

    // 2. リポジトリを使って保存
    let created = self.todo_repository.create(&todo).await?;

    // 3. レスポンス用に変換して返す
    Ok(TodoResponse::from(created))
}
```
このように、
1.  **入力の変換**（必要ならバリデーションなど）
2.  **永続化**（リポジトリの使用）
3.  **出力の変換**（DTOへの変換）
という流れを制御するのがサービスの役割です。

ビジネスルール（例：「Todoは1人100個まで」など）があれば、ここに記述します。
