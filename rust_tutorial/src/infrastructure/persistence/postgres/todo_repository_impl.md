# src/infrastructure/persistence/postgres/todo_repository_impl.rs 解説

このファイルは、**実際にデータベース（PostgreSQL）と会話してデータの保存や取得を行う** 場所です。
ドメイン層で定義された `TodoRepository` という「契約（インターフェース）」の実装クラスです。

## 構造体
```rust
pub struct PostgresTodoRepository {
    pool: PgPool,
}
```
- `PgPool`: `sqlx` が提供するデータベース接続プール。これを使ってSQLを実行します。

## SQLの実装
`impl TodoRepository for PostgresTodoRepository` ブロック内で、具体的な SQL を記述しています。

### 作成 (create)
```rust
sqlx::query_as::<_, Todo>(
    r#"
    INSERT INTO todos (...) VALUES (...)
    RETURNING ...
    "#,
)
```
- `INSERT INTO`: データを挿入します。
- `RETURNING ...`: 挿入したデータをそのまま返してもらいます（生成された日時などを取得するため）。
- `.bind(...)`: ユーザーからの入力を安全に SQL に埋め込みます（SQLインジェクション対策）。

### 検索 (find_by_id)
    async fn find_by_id(&self, id: TodoId, user_id: Uuid) -> AppResult<Option<Todo>> {
        // ...
        WHERE id = $1 AND user_id = $2
    }
```
- `id` の型は `TodoId` ですが、`#[sqlx(transparent)]` のおかげで、そのまま SQL のパラメータとして渡せます。
- `user_id` も条件に含めることで、**他人の Todo を勝手に見れないように** しています。

### 一覧・ページネーション (find_all_by_user)
```rust
    SELECT ... ORDER BY created_at DESC LIMIT $2 OFFSET $3
```
- `ORDER BY`: 作成日時の新しい順に並べます。
- `LIMIT`, `OFFSET`: ページネーション（「〇〇件目から〇〇件取得」）を実現します。

このファイルのおかげで、ビジネスロジック側（Service層）は「SQLの詳細」を知らなくても、「保存して」「検索して」と頼むだけで済みます。
