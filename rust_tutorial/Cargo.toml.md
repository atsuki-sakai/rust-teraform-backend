# Cargo.toml 解説

`Cargo.toml` は、Node.js の `package.json` に相当するファイルで、プロジェクトの設定や依存ライブラリ（クレート）を管理します。

## 全体像

```toml
[package]
name = "rust-teraform-backend"
version = "0.1.0"
edition = "2021"
default-run = "rust-teraform-backend"

[dependencies]
# ... (ライブラリのリスト)
```

## 各セクションの解説

### [package]
プロジェクトの基本情報です。
- `name`: プロジェクト名。
- `version`: バージョン番号。
- `edition`: Rust のエディション（言語仕様のバージョン）。通常は最新の `2021` を使います。

### [dependencies]
このプロジェクトで使用している主なライブラリ（クレート）の解説です。

#### Web Framework
- **axum**: 高速で使いやすい Web アプリケーションフレームワーク。
- **tokio**: 非同期処理を行うためのランタイム（エンジン）。`axum` を動かすために必要です。
- **tower-http**: HTTP 関連のミドルウェア（CORS, Trace など）を提供します。

#### Database
- **sqlx**: データベース接続ライブラリ。非同期で PostgreSQL に接続し、SQL を型安全に実行できます。
    - `features`: `postgres` (PostgreSQL用), `runtime-tokio-rustls` (Tokioランタイムで動く), `uuid`, `chrono` (日付型対応) などを有効にしています。

#### Serialization (データ形式の変換)
- **serde**: Rust のデータを JSON などに変換（シリアライズ/デシリアライズ）するためのフレームワーク。
- **serde_json**: JSON の読み書きを行うためのライブラリ。

#### Authentication (認証)
- **jsonwebtoken**: JWT (JSON Web Token) の生成と検証に使います。
- **argon2**: パスワードを安全にハッシュ化（暗号化）するためのライブラリ。

#### Validation (検証)
- **validator**: ユーザーからの入力データ（メールアドレスの形式など）をチェックするためのライブラリ。

#### Others
- **chrono**: 日付と時刻を扱うライブラリ。
- **uuid**: 一意な ID (UUID) を生成するライブラリ。
- **thiserror**, **anyhow**: エラーハンドリングを楽にするためのライブラリ。
- **tracing**: ログ出力のためのライブラリ。
- **dotenvy**: `.env` ファイルから環境変数を読み込むためのライブラリ。
- **utoipa**: コードから OpenAPI (Swagger) ドキュメントを自動生成するためのライブラリ。

## [[bin]]
```toml
[[bin]]
name = "generate_docs"
path = "src/bin/generate_docs.rs"
```
これは、メインのサーバーとは別に実行できる「サブコマンド」のようなツールを定義しています。
`cargo run --bin generate_docs` と打つことで、このファイルを実行できます。
