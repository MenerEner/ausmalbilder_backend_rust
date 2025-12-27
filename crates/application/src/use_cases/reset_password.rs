use crate::ports::TokenRepositoryError;
use crate::ports::password_hasher::PasswordHasher;
use crate::ports::password_reset_token_repository::PasswordResetTokenRepository;
use crate::ports::user_repository::UserRepository;
use std::sync::Arc;

pub struct ResetPasswordUseCase {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn PasswordResetTokenRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    expiry_hours: u64,
}

impl ResetPasswordUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_repo: Arc<dyn PasswordResetTokenRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        expiry_hours: u64,
    ) -> Self {
        Self {
            user_repo,
            token_repo,
            password_hasher,
            expiry_hours,
        }
    }

    pub async fn execute(
        &self,
        token_str: &str,
        new_password: &str,
    ) -> Result<(), ResetPasswordError> {
        let token = self
            .token_repo
            .find_by_token(token_str)
            .await?
            .ok_or(ResetPasswordError::InvalidToken)?;

        // Check expiry
        let now = chrono::Utc::now();
        let expiry_duration = chrono::Duration::hours(self.expiry_hours as i64);
        if now - token.created_at > expiry_duration {
            self.token_repo.delete_by_token(token_str).await?;
            return Err(ResetPasswordError::ExpiredToken);
        }

        let mut user = self
            .user_repo
            .find_by_id(token.user_id)
            .await?
            .ok_or(ResetPasswordError::UserNotFound)?;

        let password_hash = self.password_hasher.hash(new_password).await.map_err(|e| {
            ResetPasswordError::InternalError(format!("Failed to hash password: {}", e))
        })?;

        user.password_hash = password_hash;

        self.user_repo.update(&user).await?;
        self.token_repo.delete_by_token(token_str).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ResetPasswordError {
    InvalidToken,
    ExpiredToken,
    UserNotFound,
    RepositoryError(String),
    InternalError(String),
}

impl From<crate::ports::user_repository::UserRepositoryError> for ResetPasswordError {
    fn from(err: crate::ports::user_repository::UserRepositoryError) -> Self {
        ResetPasswordError::RepositoryError(err.to_string())
    }
}

impl From<TokenRepositoryError> for ResetPasswordError {
    fn from(err: TokenRepositoryError) -> Self {
        ResetPasswordError::RepositoryError(err.to_string())
    }
}

impl std::fmt::Display for ResetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::ExpiredToken => write!(f, "Expired token"),
            Self::UserNotFound => write!(f, "User not found"),
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ResetPasswordError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::TokenRepositoryError;
    use crate::ports::password_hasher::{PasswordHasher, PasswordHasherError};
    use crate::ports::password_reset_token_repository::{
        PasswordResetToken, PasswordResetTokenRepository,
    };
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
        async fn create(&self, _user: &User) -> Result<(), UserRepositoryError> {
            unimplemented!()
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
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, UserRepositoryError> {
            unimplemented!()
        }
        async fn find_active_by_email(
            &self,
            _email: &str,
        ) -> Result<Option<User>, UserRepositoryError> {
            unimplemented!()
        }
        async fn find_all_active(&self) -> Result<Vec<User>, UserRepositoryError> {
            unimplemented!()
        }
        async fn find_all_active_paginated(
            &self,
            _page: u64,
            _page_size: u64,
        ) -> Result<(Vec<User>, u64), UserRepositoryError> {
            unimplemented!()
        }
    }

    struct MockTokenRepository {
        tokens: Mutex<Vec<PasswordResetToken>>,
    }

    #[async_trait]
    impl PasswordResetTokenRepository for MockTokenRepository {
        async fn create(&self, _token: &PasswordResetToken) -> Result<(), TokenRepositoryError> {
            unimplemented!()
        }
        async fn find_by_token(
            &self,
            token: &str,
        ) -> Result<Option<PasswordResetToken>, TokenRepositoryError> {
            Ok(self
                .tokens
                .lock()
                .unwrap()
                .iter()
                .find(|t| t.token == token)
                .cloned())
        }
        async fn delete_by_token(&self, token: &str) -> Result<(), TokenRepositoryError> {
            let mut tokens = self.tokens.lock().unwrap();
            tokens.retain(|t| t.token != token);
            Ok(())
        }
        async fn delete_by_user_id(&self, _user_id: &Uuid) -> Result<(), TokenRepositoryError> {
            unimplemented!()
        }
    }

    struct MockPasswordHasher;
    #[async_trait]
    impl PasswordHasher for MockPasswordHasher {
        async fn hash(&self, password: &str) -> Result<String, PasswordHasherError> {
            Ok(format!("hashed_{}", password))
        }
        async fn verify(&self, _password: &str, _hash: &str) -> Result<bool, PasswordHasherError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_reset_password_success() {
        let user_id = Uuid::new_v4();
        let user = User::new(
            user_id,
            "John".to_string(),
            "Doe".to_string(),
            "john@example.com".to_string(),
            None,
            "old_hash".to_string(),
            None,
        );

        let user_repo = Arc::new(MockUserRepository {
            users: Mutex::new(vec![user]),
        });
        let token = PasswordResetToken {
            token: "token123".to_string(),
            user_id,
            created_at: chrono::Utc::now(),
        };
        let token_repo = Arc::new(MockTokenRepository {
            tokens: Mutex::new(vec![token]),
        });
        let hasher = Arc::new(MockPasswordHasher);
        let use_case = ResetPasswordUseCase::new(user_repo.clone(), token_repo.clone(), hasher, 24);

        use_case.execute("token123", "new_password").await.unwrap();

        let updated_user = user_repo.find_by_id(user_id).await.unwrap().unwrap();
        assert_eq!(updated_user.password_hash, "hashed_new_password");

        let tokens = token_repo.tokens.lock().unwrap();
        assert!(tokens.is_empty());
    }

    #[tokio::test]
    async fn test_reset_password_expired_token() {
        let user_id = Uuid::new_v4();
        let user = User::new(
            user_id,
            "John".to_string(),
            "Doe".to_string(),
            "john@example.com".to_string(),
            None,
            "old_hash".to_string(),
            None,
        );

        let user_repo = Arc::new(MockUserRepository {
            users: Mutex::new(vec![user]),
        });
        let token = PasswordResetToken {
            token: "token123".to_string(),
            user_id,
            created_at: chrono::Utc::now() - chrono::Duration::hours(25),
        };
        let token_repo = Arc::new(MockTokenRepository {
            tokens: Mutex::new(vec![token]),
        });
        let hasher = Arc::new(MockPasswordHasher);
        let use_case = ResetPasswordUseCase::new(user_repo.clone(), token_repo.clone(), hasher, 24);

        let result = use_case.execute("token123", "new_password").await;
        assert!(matches!(result, Err(ResetPasswordError::ExpiredToken)));

        let tokens = token_repo.tokens.lock().unwrap();
        assert!(tokens.is_empty());
    }
}
