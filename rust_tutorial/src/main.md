# src/main.rs 解説

`src/main.rs` は、Rust のプログラムにおいて **一番最初に実行される「エントリーポイント」** です。
Web サーバーの起動準備と実行を行います。

## 1. 宣言部 (Imports)
```rust
use axum::Router;
// ...
```
必要なライブラリやモジュールを読み込んでいます。
- `axum::Router`: Web サーバーのルーティング（URLごとの処理の割り振り）を定義します。
- `AppState`: データベース接続など、アプリ全体で共有したいデータです。

## 2. Main 関数
```rust
#[tokio::main]
async fn main() {
    // ...
}
```
- `fn main()`: プログラムの開始地点です。
- `#[tokio::main]`: 非同期処理 (`async/await`) を使えるようにする魔法のスイッチ（マクロ）です。

## 3. 初期化処理
```rust
tracing_subscriber::registry()... // ログ設定
let state = AppState::new().await... // 共有データ(DB接続)の準備
```
サーバーが動く前の準備運動です。ログを出せるようにしたり、データベースに接続したりします。

## 4. ルーティング設定 (Router)
```rust
let app = Router::new()
    .route("/health", axum::routing::get(health_check))
    .nest("/api/v1/auth", auth_routes())
    .nest("/api/v1/todos", todo_routes(state.clone()))
    // ...
```
ここがサーバーの設計図です。
- `.route("/health", ...)`: `/health` にアクセスが来たら `health_check` 関数を実行。
- `.nest(...)`: URL をグループ化します。例えば `/api/v1/todos` 以下の処理は `todo_routes` に任せます。
- `.with_state(state)`: データベース接続情報 (`state`) をすべてのハンドラで使えるように渡します。

## 5. サーバー起動
```rust
let port: u16 = std::env::var("PORT")...
let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
axum::serve(listener, app).await.unwrap();
```
- ポート番号（デフォルト8080）を決定します。
- `axum::serve` でサーバーを起動し、リクエストが来るのを待ち続けます。
