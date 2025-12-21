use chrono::NaiveDate;
use crate::ports::password_hasher::PasswordHasher;
use crate::ports::user_repository::UserRepository;
use domain_users::User;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl CreateUserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repo,
            password_hasher,
        }
    }

    pub async fn execute(&self, input: CreateUserInput) -> Result<User, CreateUserError> {
        if self
            .user_repo
            .find_active_by_email(&input.email)
            .await?
            .is_some()
        {
            return Err(CreateUserError::AlreadyExists(input.email));
        }

        let password_hash = self
            .password_hasher
            .hash(&input.password)
            .await
            .map_err(|e| {
                CreateUserError::InternalError(format!("Failed to hash password: {}", e))
            })?;

        let user = User::new(
            Uuid::new_v4(),
            input.first_name,
            input.last_name,
            input.email,
            input.phone_number,
            password_hash,
            input.birth_date,
        );

        self.user_repo.create(&user).await?;

        Ok(user)
    }
}

#[derive(Clone)]
pub struct CreateUserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Debug)]
pub enum CreateUserError {
    AlreadyExists(String),
    RepositoryError(String),
    InternalError(String),
}

impl From<crate::ports::user_repository::UserRepositoryError> for CreateUserError {
    fn from(err: crate::ports::user_repository::UserRepositoryError) -> Self {
        match err {
            crate::ports::user_repository::UserRepositoryError::AlreadyExists(email) => {
                Self::AlreadyExists(email)
            }
            crate::ports::user_repository::UserRepositoryError::DatabaseError(msg) => {
                Self::RepositoryError(msg)
            }
            crate::ports::user_repository::UserRepositoryError::NotFound(msg) => {
                Self::RepositoryError(msg)
            }
        }
    }
}

impl std::fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists(email) => write!(f, "User with email {} already exists", email),
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CreateUserError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::password_hasher::PasswordHasher;
    use crate::ports::user_repository::{UserRepository, UserRepositoryError};
    use async_trait::async_trait;
    use domain_users::User;
    use std::sync::Mutex;
    use uuid::Uuid;

    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: &User) -> Result<(), UserRepositoryError> {
            self.users.lock().unwrap().push(user.clone());
            Ok(())
        }
        async fn update(&self, user: &User) -> Result<(), UserRepositoryError> {
            let mut users = self.users.lock().unwrap();
            if let Some(u) = users.iter_mut().find(|u| u.id == user.id) {
                *u = user.clone();
                Ok(())
            } else {
                Err(UserRepositoryError::NotFound(user.id.to_string()))
            }
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.id == id)
                .cloned())
        }
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.email == email)
                .cloned())
        }
        async fn find_active_by_email(
            &self,
            email: &str,
        ) -> Result<Option<User>, UserRepositoryError> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.email == email && !u.is_deleted())
                .cloned())
        }
        async fn find_all_active(&self) -> Result<Vec<User>, UserRepositoryError> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .filter(|u| !u.is_deleted())
                .cloned()
                .collect())
        }
        async fn find_all_active_paginated(
            &self,
            page: u64,
            page_size: u64,
        ) -> Result<(Vec<User>, u64), UserRepositoryError> {
            let users = self.users.lock().unwrap();
            let active_users: Vec<User> =
                users.iter().filter(|u| !u.is_deleted()).cloned().collect();
            let total = active_users.len() as u64;
            let offset = (page * page_size) as usize;
            let paged_users = active_users
                .into_iter()
                .skip(offset)
                .take(page_size as usize)
                .collect();
            Ok((paged_users, total))
        }
    }

    struct MockPasswordHasher;
    #[async_trait]
    impl PasswordHasher for MockPasswordHasher {
        async fn hash(
            &self,
            password: &str,
        ) -> Result<String, crate::ports::password_hasher::PasswordHasherError> {
            Ok(password.to_string())
        }
        async fn verify(
            &self,
            _password: &str,
            _hash: &str,
        ) -> Result<bool, crate::ports::password_hasher::PasswordHasherError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_create_user_after_soft_delete() {
        let repo = Arc::new(MockUserRepository {
            users: Mutex::new(vec![]),
        });
        let hasher = Arc::new(MockPasswordHasher);
        let use_case = CreateUserUseCase::new(repo.clone(), hasher);

        let input = CreateUserInput {
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: "test@example.com".to_string(),
            phone_number: None,
            password: "password".to_string(),
            birth_date: None,
        };

        // Create first user
        let user1 = use_case.execute(input.clone()).await.unwrap();

        // Soft delete first user
        let mut user1_deleted = user1.clone();
        user1_deleted.delete();
        repo.update(&user1_deleted).await.unwrap();

        // Create second user with same email
        let user2 = use_case.execute(input).await.unwrap();

        assert_ne!(user1.id, user2.id);
        assert_eq!(user1.email, user2.email);
        assert!(!user2.is_deleted());
    }
}
