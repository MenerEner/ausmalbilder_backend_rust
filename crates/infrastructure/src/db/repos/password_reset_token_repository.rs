use crate::db::entities::password_reset_token::{ActiveModel, Column, Entity as TokenEntity};
use application::ports::TokenRepositoryError;
use application::ports::password_reset_token_repository::{
    PasswordResetToken, PasswordResetTokenRepository,
};
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

pub struct PostgresPasswordResetTokenRepository {
    db: DatabaseConnection,
}

impl PostgresPasswordResetTokenRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PasswordResetTokenRepository for PostgresPasswordResetTokenRepository {
    async fn create(&self, token: &PasswordResetToken) -> Result<(), TokenRepositoryError> {
        let active_model = ActiveModel {
            token: Set(token.token.clone()),
            user_id: Set(token.user_id),
            created_at: Set(token.created_at.into()),
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
    ) -> Result<Option<PasswordResetToken>, TokenRepositoryError> {
        let db_token = TokenEntity::find_by_id(token)
            .one(&self.db)
            .await
            .map_err(|e| TokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(db_token.map(|t| PasswordResetToken {
            token: t.token,
            user_id: t.user_id,
            created_at: t.created_at.into(),
        }))
    }

    async fn delete_by_token(&self, token: &str) -> Result<(), TokenRepositoryError> {
        TokenEntity::delete_by_id(token)
            .exec(&self.db)
            .await
            .map_err(|e| TokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), TokenRepositoryError> {
        TokenEntity::delete_many()
            .filter(Column::UserId.eq(*user_id))
            .exec(&self.db)
            .await
            .map_err(|e| TokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
