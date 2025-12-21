use crate::ports::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::ports::user_repository::UserRepository;
use std::sync::Arc;

pub struct VerifyEmailUseCase {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn EmailVerificationTokenRepository>,
}

impl VerifyEmailUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_repo: Arc<dyn EmailVerificationTokenRepository>,
    ) -> Self {
        Self {
            user_repo,
            token_repo,
        }
    }

    pub async fn execute(&self, token_str: &str) -> Result<(), VerifyEmailError> {
        let token = self
            .token_repo
            .find_by_token(token_str)
            .await?
            .ok_or(VerifyEmailError::InvalidToken)?;

        let mut user = self
            .user_repo
            .find_by_id(token.user_id)
            .await?
            .ok_or(VerifyEmailError::UserNotFound)?;

        user.verify_email();

        self.user_repo.update(&user).await?;

        self.token_repo.delete_by_token(token_str).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum VerifyEmailError {
    InvalidToken,
    UserNotFound,
    RepositoryError(String),
}

impl From<crate::ports::user_repository::UserRepositoryError> for VerifyEmailError {
    fn from(err: crate::ports::user_repository::UserRepositoryError) -> Self {
        VerifyEmailError::RepositoryError(err.to_string())
    }
}

impl From<crate::ports::email_verification_token_repository::TokenRepositoryError> for VerifyEmailError {
    fn from(err: crate::ports::email_verification_token_repository::TokenRepositoryError) -> Self {
        VerifyEmailError::RepositoryError(err.to_string())
    }
}

impl std::fmt::Display for VerifyEmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken => write!(f, "Invalid or expired token"),
            Self::UserNotFound => write!(f, "User not found"),
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
        }
    }
}

impl std::error::Error for VerifyEmailError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::email_verification_token_repository::{
        EmailVerificationToken, EmailVerificationTokenRepository, TokenRepositoryError,
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
            Ok(self.users.lock().unwrap().iter().find(|u| u.id == id).cloned())
        }
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, UserRepositoryError> {
            unimplemented!()
        }
        async fn find_active_by_email(&self, _email: &str) -> Result<Option<User>, UserRepositoryError> {
            unimplemented!()
        }
        async fn find_all_active(&self) -> Result<Vec<User>, UserRepositoryError> {
            unimplemented!()
        }
    }

    struct MockTokenRepository {
        tokens: Mutex<Vec<EmailVerificationToken>>,
    }

    #[async_trait]
    impl EmailVerificationTokenRepository for MockTokenRepository {
        async fn create(&self, token: &EmailVerificationToken) -> Result<(), TokenRepositoryError> {
            self.tokens.lock().unwrap().push(token.clone());
            Ok(())
        }
        async fn find_by_token(&self, token: &str) -> Result<Option<EmailVerificationToken>, TokenRepositoryError> {
            Ok(self.tokens.lock().unwrap().iter().find(|t| t.token == token).cloned())
        }
        async fn delete_by_token(&self, token: &str) -> Result<(), TokenRepositoryError> {
            let mut tokens = self.tokens.lock().unwrap();
            tokens.retain(|t| t.token != token);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_verify_email_success() {
        let user_id = Uuid::new_v4();
        let user = User::new(
            user_id,
            "John".to_string(),
            "Doe".to_string(),
            "john@example.com".to_string(),
            None,
            "hash".to_string(),
            None,
        );

        let user_repo = Arc::new(MockUserRepository { users: Mutex::new(vec![user]) });
        let token = EmailVerificationToken { token: "token123".to_string(), user_id };
        let token_repo = Arc::new(MockTokenRepository { tokens: Mutex::new(vec![token]) });
        let use_case = VerifyEmailUseCase::new(user_repo.clone(), token_repo.clone());

        use_case.execute("token123").await.unwrap();

        let updated_user = user_repo.find_by_id(user_id).await.unwrap().unwrap();
        assert!(updated_user.is_email_verified);
        assert_eq!(updated_user.role, domain_users::models::user::UserRole::VerifiedUser);

        let tokens = token_repo.tokens.lock().unwrap();
        assert!(tokens.is_empty());
    }
}
