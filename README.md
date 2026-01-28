# Rust Terraform Backend (Todo API)

これは、クリーンアーキテクチャの原則に従って Rust で構築された RESTful Todo API バックエンドです。

## 🏗️ 技術スタック

- **Core**: Rust (Tokio)
- **Web Framework**: Axum
- **Database**: PostgreSQL (SQLx 経由でアクセス)
- **Serialization**: Serde
- **Authentication**: JWT & Argon2
- **Documentation**: Utoipa (Swagger UI)
- **Infrastructure**: Docker Compose

## 🏛️ アーキテクチャ (クリーンアーキテクチャ)

このプロジェクトは、関心の分離を行うためにレイヤー構造になっています：

- **`src/presentation`**: API ハンドラ、ルーティング、ミドルウェア（インターフェース・アダプター層）
- **`src/application`**: ビジネスロジック、ユースケース（Services）、DTO（アプリケーション・ロジック層）
- **`src/domain`**: エンティティとリポジトリのインターフェース（エンタープライズ・ビジネスルール層）
- **`src/infrastructure`**: データベースの実装、設定、外部サービス（フレームワーク＆ドライバ層）

## 🚀 はじめ方

### 前提条件

- Rust (最新の安定版)
- Docker & Docker Compose
- `sqlx-cli` (オプション: 手動でマイグレーションを実行する場合)

### アプリの実行

1. **データベースの起動**:
   ```bash
   docker-compose up -d
   ```

2. **マイグレーションの実行** (起動時に自動化されていない場合):
   ```bash
   sqlx migrate run
   ```

3. **サーバーの起動**:
   ```bash
   cargo run
   ```

サーバーは `http://localhost:8080` で起動します。

### 📚 API ドキュメント

サーバー起動後、以下にアクセスしてください：
- **Swagger UI**: `http://localhost:8080/swagger-ui`

## ☁️ クラウドインフラ (AWS / GCP)

このプロジェクトは、**Terraform** で管理された **Google Cloud Platform (GCP)** 上で動作するように設計されています。

### アーキテクチャ図

```mermaid
graph TD
    User([User]) -->|HTTPS| CloudRun[Cloud Run<br>(App Server)]
    CloudRun -->|Private IP| CloudSQL[(Cloud SQL<br>PostgreSQL)]
    CloudRun -->|Get Config| SecretManager[Secret Manager]

    subgraph GCP Project
        CloudRun
        CloudSQL
        SecretManager
    end
```

### 主要コンポーネント

- **Cloud Run**: Rust アプリケーションをホストするためのサーバーレスコンテナプラットフォーム。トラフィックに応じて自動的にスケールします。
- **Cloud SQL (PostgreSQL)**: マネージドのリレーショナルデータベース。アクセスはプライベート IP (VPC ピアリング) 経由で保護されており、パブリックインターネットには公開されません。
- **Secret Manager**: `DATABASE_URL` や `JWT_SECRET` などの機密情報を安全に保存します。アプリケーションは実行時にこれらを取得します。

### Infrastructure as Code (Terraform)

すべてのインフラは `terraform/` ディレクトリで定義されています。
Terraform コードの詳細な解説については、**[rust_tutorial/terraform.md](rust_tutorial/terraform.md)** をご覧ください。

## 📖 初心者向け Rust チュートリアル

このコードベースのファイルごとの詳細な解説については、`rust_tutorial/` ディレクトリをチェックしてください。ソースコードの構造をミラーリングしており、各ファイルの目的を説明しています。
