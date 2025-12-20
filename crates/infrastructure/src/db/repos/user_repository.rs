use crate::db::entities::user::{Entity as UserEntity, ActiveModel as UserActiveModel};
use crate::db::mapper::UserMapper;
use application::ports::user_repository::{UserRepository, UserRepositoryError};
use async_trait::async_trait;
use domain_users::User;
use sea_orm::*;
use uuid::Uuid;

pub struct PostgresUserRepository {
    db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<(), UserRepositoryError> {
        let active_model = UserActiveModel {
            id: Set(user.id),
            name: Set(user.name.clone()),
            email: Set(user.email.clone()),
            phone_number: Set(user.phone_number.clone()),
            password_hash: Set(user.password_hash.clone()),
        };

        UserEntity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| {
                let err_msg = e.to_string();
                if err_msg.contains("duplicate key value") || err_msg.contains("UNIQUE constraint failed") {
                    UserRepositoryError::AlreadyExists(user.email.clone())
                } else {
                    UserRepositoryError::DatabaseError(err_msg)
                }
            })?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError> {
        let db_user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(db_user.map(UserMapper::to_domain))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let db_user = UserEntity::find()
            .filter(crate::db::entities::user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(db_user.map(UserMapper::to_domain))
    }
}
