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
  - **`value_objects`**: 型安全性を保証する NewType パターンの実装
- **`src/infrastructure`**: データベースの実装、設定、外部サービス（フレームワーク＆ドライバ層）

## 🔒 型安全性と NewType パターン

このプロジェクトでは、プリミティブ型の直接使用を避け、**NewType パターン**を採用することで型安全性とバグ防止を実現しています。

### NewType パターンとは

NewType パターンは、プリミティブ型（`String`, `Uuid`, `i64`, `bool` など）を専用の型でラップすることで、コンパイル時に型の誤用を防ぐ設計パターンです。

### 実装されている Value Objects

#### User 関連型 ([user_types.rs](src/domain/value_objects/user_types.rs))
- **`UserId`**: `Uuid` をラップし、ユーザー識別子として型安全に使用
- **`Email`**: メール形式バリデーション付き文字列型（自動的に小文字化、`@` と `.` の検証）
- **`PasswordHash`**: パスワードハッシュを保持する型（シリアライズ時には非公開）

#### Todo 関連型 ([todo_types.rs](src/domain/value_objects/todo_types.rs))
- **`TodoId`**: `Uuid` をラップし、Todo 識別子として型安全に使用
- **`TodoTitle`**: 1〜255文字の検証付きタイトル型
- **`TodoDescription`**: 最大1000文字の検証付き説明文型
- **`CompletionStatus`**: `Pending` / `Completed` の2状態を持つ enum（DB では boolean で保存）

#### DTO 関連型 ([todo_dto.rs](src/application/dto/todo_dto.rs))
- **`Page`**: ページ番号（最小値: 1）
- **`PerPage`**: 1ページあたりのアイテム数（範囲: 1〜100）
- **`Offset`**: ページとアイテム数から自動計算されるオフセット値

### NewType の利点

```rust
// ❌ 間違った使い方（プリミティブ型を直接使用）
fn update_todo(user_id: Uuid, todo_id: Uuid) { ... }
// 引数の順序を間違えてもコンパイルエラーにならない！
update_todo(todo_id, user_id); // コンパイル成功、実行時エラー

// ✅ 正しい使い方（NewType パターン）
fn update_todo(user_id: UserId, todo_id: TodoId) { ... }
// 引数の順序を間違えるとコンパイルエラーになる！
update_todo(todo_id, user_id); // コンパイルエラー！
```

### バリデーション例

```rust
// Email の作成とバリデーション
let email = Email::new("user@example.com".to_string())?; // ✅ OK
let invalid = Email::new("invalid".to_string()?; // ❌ Err("Invalid email format")

// TodoTitle の長さ検証
let title = TodoTitle::new("買い物".to_string())?; // ✅ OK
let too_long = TodoTitle::new("あ".repeat(256))?; // ❌ Err("Title cannot be longer than 255 characters")

// CompletionStatus の使用
let status = CompletionStatus::Pending;
status.toggle(); // CompletionStatus::Completed に変わる
status.is_completed(); // true
```

### SQLx との統合

NewType は `#[sqlx(transparent)]` を使用することで、データベースとシームレスに統合されます：

```rust
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct TodoTitle(String);

// SQLx は自動的に String として保存/読み込みを行う
```

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

#### Terraform セットアップ

1. **GCS バケットの作成（Terraform state 管理用）**:
   ```bash
   export PROJECT_ID=your-gcp-project-id
   gsutil mb -p ${PROJECT_ID} -l asia-northeast1 gs://${PROJECT_ID}-terraform-state
   gsutil versioning set on gs://${PROJECT_ID}-terraform-state
   ```

2. **Terraform の初期化**:
   ```bash
   cd terraform
   terraform init
   ```

3. **インフラのデプロイ**:
   ```bash
   terraform plan
   terraform apply
   ```

#### CI/CD (GitHub Actions) のセットアップ

このプロジェクトでは、セキュリティ向上のため **Workload Identity Federation** を使用しています。

**必要な GitHub Secrets**:
- `WIF_PROVIDER`: Workload Identity プロバイダーの完全なリソース名
  - 例: `projects/123456789/locations/global/workloadIdentityPools/github-pool/providers/github-provider`
- `WIF_SERVICE_ACCOUNT`: デプロイに使用するサービスアカウント
  - 例: `deploy@your-project.iam.gserviceaccount.com`
- `GCP_PROJECT_ID`: GCP プロジェクト ID

**Workload Identity の設定手順**:
```bash
# Workload Identity Pool の作成
gcloud iam workload-identity-pools create github-pool \
  --location=global \
  --display-name="GitHub Actions Pool"

# Provider の作成
gcloud iam workload-identity-pools providers create-oidc github-provider \
  --location=global \
  --workload-identity-pool=github-pool \
  --issuer-uri=https://token.actions.githubusercontent.com \
  --attribute-mapping="google.subject=assertion.sub,attribute.repository=assertion.repository" \
  --attribute-condition="assertion.repository=='your-org/your-repo'"

# サービスアカウントへの権限付与
gcloud iam service-accounts add-iam-policy-binding deploy@your-project.iam.gserviceaccount.com \
  --role=roles/iam.workloadIdentityUser \
  --member="principalSet://iam.googleapis.com/projects/PROJECT_NUMBER/locations/global/workloadIdentityPools/github-pool/attribute.repository/your-org/your-repo"
```

## 📖 初心者向け Rust チュートリアル

このコードベースのファイルごとの詳細な解説については、`rust_tutorial/` ディレクトリをチェックしてください。ソースコードの構造をミラーリングしており、各ファイルの目的を説明しています。
