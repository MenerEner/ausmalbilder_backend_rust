use crate::ports::TokenRepositoryError;
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
