# Terraform ディレクトリ (`terraform/`) 解説

このディレクトリには、Google Cloud Platform (GCP) 上にアプリケーションをデプロイするための「インフラ定義書」が格納されています。
Terraform を使うと、画面（コンソール）でポチポチ操作する代わりに、コードでインフラを構築・管理できます（**Infrastructure as Code**）。

## 主要な構成要素

このプロジェクトでは、主に以下の3つの GCP サービスを利用します。

1.  **Cloud Run**: アプリケーション（Webサーバー）を動かす場所。
2.  **Cloud SQL (PostgreSQL)**: データベース。
3.  **Secret Manager**: パスワードや鍵などの機密情報を安全に保存する場所。

---

## 各ファイルの役割

### 1. `main.tf` (全体設定)
Terraformの基本設定や、プロバイダー（GCPを使うよ、という宣言）が書かれています。
```hcl
provider "google" {
  project = var.project_id
  region  = var.region
}
```

### 2. `cloud_run.tf` (アプリの実行環境)
Web アプリケーション自体（コンテナ）の設定です。

- **Autoscaling (自動スケーリング)**:
    ```hcl
    min_instance_count = var.cloud_run_min_instances
    max_instance_count = var.cloud_run_max_instances
    ```
    アクセスが増えたら自動でサーバーの台数を増やし、減ったら減らします。

- **環境変数とSecret**:
    ```hcl
    env {
      name = "DATABASE_URL"
      value_source { ... } # Secret Manager から値を取得
    }
    ```
    コードの中にパスワードを書かず、デプロイ時に安全な場所から引っ張ってくる仕組みです。

- **VPC Access**:
    データベースはセキュリティのためインターネットから隔離されています。Cloud Run がそこにアクセスするための「専用通路（コネクタ）」を通す設定です。

### 3. `cloud_sql.tf` (データベース)
PostgreSQL データベースの設定です。

- **Private IP (プライベートIP)**:
    ```hcl
    ipv4_enabled    = false
    private_network = ...
    ```
    インターネット（パブリックIP）からの接続を無効化し、プロジェクト内部のネットワークからしか繋げないようにしてセキュリティを高めています。

- **Database & User**:
    自動的にデータベースの名前とユーザーを作成します。

### 4. `vpc.tf` (ネットワーク)
データベースとアプリを繋ぐための「内部ネットワーク」の設定です。

### 5. `variables.tf` (変数)
設定値をまとめて管理するファイルです。
例えば「リージョン（場所）」を `asia-northeast1` (東京) から別の場所に変えたい時は、ここ（または `tfvars` ファイル）を変更するだけで済みます。

---

## インフラ構築の流れ

1.  **`terraform init`**: 準備（必要なプラグインのダウンロード）。
2.  **`terraform plan`**: 計画（これから何を作るか、変更するかを表示）。
3.  **`terraform apply`**: 実行（実際に GCP 上にリソースを作成）。

この設定により、誰が実行しても同じ構成のインフラが再現できるようになっています。
