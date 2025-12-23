use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EmailVerificationToken {
    pub token: String,
    pub user_id: Uuid,
}

#[async_trait]
pub trait EmailVerificationTokenRepository: Send + Sync {
    async fn create(&self, token: &EmailVerificationToken) -> Result<(), TokenRepositoryError>;
    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<EmailVerificationToken>, TokenRepositoryError>;
    async fn delete_by_token(&self, token: &str) -> Result<(), TokenRepositoryError>;
}

#[derive(Debug)]
pub enum TokenRepositoryError {
    DatabaseError(String),
}

impl std::fmt::Display for TokenRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for TokenRepositoryError {}
