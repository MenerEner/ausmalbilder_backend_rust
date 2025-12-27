use crate::ports::TokenRepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PasswordResetToken {
    pub token: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait PasswordResetTokenRepository: Send + Sync {
    async fn create(&self, token: &PasswordResetToken) -> Result<(), TokenRepositoryError>;
    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<PasswordResetToken>, TokenRepositoryError>;
    async fn delete_by_token(&self, token: &str) -> Result<(), TokenRepositoryError>;
    async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), TokenRepositoryError>;
}
