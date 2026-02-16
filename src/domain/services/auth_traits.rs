use crate::domain::value_objects::{Email, UserId};
use crate::shared::error::AppResult;

/// パスワードのハッシュ化と検証を担当するトレイト
///
/// Application層がInfrastructure層の具体的なハッシュ化アルゴリズム（Argon2等）に
/// 依存しないようにするための抽象化層
pub trait PasswordHasher: Send + Sync {
    /// パスワードをハッシュ化する
    fn hash(&self, password: &str) -> AppResult<String>;

    /// パスワードとハッシュが一致するか検証する
    fn verify(&self, password: &str, hash: &str) -> AppResult<bool>;
}

/// JWTトークンの生成と検証を担当するトレイト
///
/// Application層がInfrastructure層の具体的なJWT実装に依存しないようにするための抽象化層
pub trait TokenGenerator: Send + Sync {
    /// アクセストークンを生成する
    fn generate_access_token(&self, user_id: UserId, email: &Email) -> AppResult<String>;

    /// リフレッシュトークンを生成する
    fn generate_refresh_token(&self, user_id: UserId, email: &Email) -> AppResult<String>;

    /// トークンを検証してクレームを取得する
    fn verify_token(&self, token: &str) -> AppResult<TokenClaims>;

    /// アクセストークンの有効期限（秒）を取得する
    fn access_token_expires_in(&self) -> i64;
}

/// トークンのクレーム情報
#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub user_id: UserId,
    pub email: String,
    pub token_type: TokenTypeEnum,
}

/// トークンの種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTypeEnum {
    Access,
    Refresh,
}
