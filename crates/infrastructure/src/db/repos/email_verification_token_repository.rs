use crate::db::entities::email_verification_token::{ActiveModel, Entity as TokenEntity};
use application::ports::TokenRepositoryError;
use application::ports::email_verification_token_repository::{
    EmailVerificationToken, EmailVerificationTokenRepository,
};
use async_trait::async_trait;
use sea_orm::*;

pub struct PostgresEmailVerificationTokenRepository {
    db: DatabaseConnection,
}

impl PostgresEmailVerificationTokenRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EmailVerificationTokenRepository for PostgresEmailVerificationTokenRepository {
    async fn create(&self, token: &EmailVerificationToken) -> Result<(), TokenRepositoryError> {
        let active_model = ActiveModel {
            token: Set(token.token.clone()),
            user_id: Set(token.user_id),
            created_at: Set(chrono::Utc::now().into()),
        };

        TokenEntity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| TokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<EmailVerificationToken>, TokenRepositoryError> {
        let db_token = TokenEntity::find_by_id(token)
            .one(&self.db)
            .await
            .map_err(|e| TokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(db_token.map(|t| EmailVerificationToken {
            token: t.token,
            user_id: t.user_id,
        }))
    }

    async fn delete_by_token(&self, token: &str) -> Result<(), TokenRepositoryError> {
        TokenEntity::delete_by_id(token)
            .exec(&self.db)
            .await
            .map_err(|e| TokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
