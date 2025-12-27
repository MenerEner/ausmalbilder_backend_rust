use crate::db::entities::user::{ActiveModel as UserActiveModel, Entity as UserEntity};
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
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email: Set(user.email.clone()),
            phone_number: Set(user.phone_number.clone()),
            password_hash: Set(user.password_hash.clone()),
            birth_date: Set(user.birth_date),
            is_email_verified: Set(user.is_email_verified),
            role: Set(user.role.to_string()),
            deleted_at: Set(user.deleted_at.map(|dt| dt.into())),
        };

        UserEntity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| {
                let err_msg = e.to_string();
                if err_msg.contains("duplicate key value")
                    || err_msg.contains("UNIQUE constraint failed")
                {
                    UserRepositoryError::AlreadyExists(user.email.clone())
                } else {
                    UserRepositoryError::DatabaseError(err_msg)
                }
            })?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), UserRepositoryError> {
        let active_model = UserActiveModel {
            id: Set(user.id),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email: Set(user.email.clone()),
            phone_number: Set(user.phone_number.clone()),
            password_hash: Set(user.password_hash.clone()),
            birth_date: Set(user.birth_date),
            is_email_verified: Set(user.is_email_verified),
            role: Set(user.role.to_string()),
            deleted_at: Set(user.deleted_at.map(|dt| dt.into())),
        };

        active_model
            .update(&self.db)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

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

    async fn find_active_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let db_user = UserEntity::find()
            .filter(crate::db::entities::user::Column::Email.eq(email))
            .filter(crate::db::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(db_user.map(UserMapper::to_domain))
    }

    async fn find_all_active(&self) -> Result<Vec<User>, UserRepositoryError> {
        let db_users = UserEntity::find()
            .filter(crate::db::entities::user::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(db_users.into_iter().map(UserMapper::to_domain).collect())
    }

    async fn find_all_active_paginated(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<User>, u64), UserRepositoryError> {
        let paginator = UserEntity::find()
            .filter(crate::db::entities::user::Column::DeletedAt.is_null())
            .paginate(&self.db, page_size);

        let total_items = paginator
            .num_items()
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let db_users = paginator
            .fetch_page(page) // SeaORM uses 0-based indexing for pages
            .await
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok((
            db_users.into_iter().map(UserMapper::to_domain).collect(),
            total_items,
        ))
    }
}
