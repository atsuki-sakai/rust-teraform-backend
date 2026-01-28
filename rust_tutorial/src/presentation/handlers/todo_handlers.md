# src/presentation/handlers/todo_handlers.rs 解説

このファイルは、**HTTP リクエストの窓口（ハンドラ）** です。
実際に Web から来たデータを Axum の機能を使って受け取り、サービス（Service）に仕事を依頼し、その結果を JSON でクライアントに返します。

## 共通の引数
どの関数も、だいたい以下のような引数を受け取っています。
```rust
pub async fn list_todos(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ...
)
```
- `State(state)`: アプリ全体で共有しているデータ（DB接続など）を取り出します。
- `Extension(claims)`: 認証ミドルウェアがチェック済みの「ログインユーザー情報」です。ここから `claims.sub` (ユーザーID) を取得できます。

## 各ハンドラの実装
### list_todos (一覧取得)
`Query(pagination)` で URL のクエリパラメータ（例 `?page=2`）を受け取っています。
```rust
let service = TodoService::new(state.todo_repository.clone());
let response = service.list(claims.sub, pagination).await?;
Ok(Json(response))
```
1.  サービスを作成（ここでリポジトリを渡します）。
2.  サービスの `list` メソッドを実行。
3.  結果を `Json(...)` で包んでレスポンスします。

### create_todo (新規作成)
`Json(request)` でリクエストボディ（JSONデータ）を受け取り、Rust の構造体 `CreateTodoRequest` に自動変換しています。

## API ドキュメント (utoipa)
```rust
#[utoipa::path(...)]
```
この巨大なアノテーションは、ここから自動的に Swagger (OpenAPI) ドキュメントを生成するためのものです。
- `path`: URL
- `params`: パラメータの説明
- `responses`: 返ってくる可能性のあるレスポンス（成功200, 未認証401など）
これ記述しておくことで、コードとドキュメントが乖離するのを防げます。
