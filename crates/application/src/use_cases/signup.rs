use crate::ports::email_service::EmailService;
use crate::ports::email_verification_token_repository::{
    EmailVerificationToken, EmailVerificationTokenRepository,
};
use crate::ports::password_hasher::PasswordHasher;
use crate::ports::user_repository::UserRepository;
use chrono::NaiveDate;
use domain_users::User;
use std::sync::Arc;
use uuid::Uuid;

pub struct SignupUseCase {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn EmailVerificationTokenRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    email_service: Arc<dyn EmailService>,
}

impl SignupUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_repo: Arc<dyn EmailVerificationTokenRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            user_repo,
            token_repo,
            password_hasher,
            email_service,
        }
    }

    pub async fn execute(&self, input: SignupInput) -> Result<User, SignupError> {
        if self
            .user_repo
            .find_active_by_email(&input.email)
            .await?
            .is_some()
        {
            return Err(SignupError::AlreadyExists(input.email));
        }

        let password_hash = self
            .password_hasher
            .hash(&input.password)
            .await
            .map_err(|e| SignupError::InternalError(format!("Failed to hash password: {}", e)))?;

        let user = User::new(
            Uuid::new_v4(),
            input.first_name,
            input.last_name,
            input.email.clone(),
            input.phone_number,
            password_hash,
            input.birth_date,
        );

        self.user_repo.create(&user).await?;

        // Generate token (for simplicity using UUID, can be more complex)
        let token_str = Uuid::new_v4().to_string();
        let token = EmailVerificationToken {
            token: token_str.clone(),
            user_id: user.id,
        };

        self.token_repo.create(&token).await?;

        self.email_service
            .send_verification_email(&input.email, &token_str)
            .await?;

        Ok(user)
    }
}

pub struct SignupInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Debug)]
pub enum SignupError {
    AlreadyExists(String),
    RepositoryError(String),
    EmailError(String),
    InternalError(String),
}

impl From<crate::ports::user_repository::UserRepositoryError> for SignupError {
    fn from(err: crate::ports::user_repository::UserRepositoryError) -> Self {
        SignupError::RepositoryError(err.to_string())
    }
}

impl From<crate::ports::email_verification_token_repository::TokenRepositoryError> for SignupError {
    fn from(err: crate::ports::email_verification_token_repository::TokenRepositoryError) -> Self {
        SignupError::RepositoryError(err.to_string())
    }
}

impl From<crate::ports::email_service::EmailError> for SignupError {
    fn from(err: crate::ports::email_service::EmailError) -> Self {
        SignupError::EmailError(err.to_string())
    }
}

impl std::fmt::Display for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists(email) => write!(f, "User with email {} already exists", email),
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            Self::EmailError(msg) => write!(f, "Email error: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SignupError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::email_service::{EmailError, EmailService};
    use crate::ports::email_verification_token_repository::{
        EmailVerificationToken, EmailVerificationTokenRepository, TokenRepositoryError,
    };
    use crate::ports::password_hasher::{PasswordHasher, PasswordHasherError};
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

    struct MockTokenRepository {
        tokens: Mutex<Vec<EmailVerificationToken>>,
    }

    #[async_trait]
    impl EmailVerificationTokenRepository for MockTokenRepository {
        async fn create(&self, token: &EmailVerificationToken) -> Result<(), TokenRepositoryError> {
            self.tokens.lock().unwrap().push(token.clone());
            Ok(())
        }
        async fn find_by_token(
            &self,
            token: &str,
        ) -> Result<Option<EmailVerificationToken>, TokenRepositoryError> {
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
    }

    struct MockPasswordHasher;
    #[async_trait]
    impl PasswordHasher for MockPasswordHasher {
        async fn hash(&self, password: &str) -> Result<String, PasswordHasherError> {
            Ok(password.to_string())
        }
        async fn verify(&self, _password: &str, _hash: &str) -> Result<bool, PasswordHasherError> {
            Ok(true)
        }
    }

    struct MockEmailService;
    #[async_trait]
    impl EmailService for MockEmailService {
        async fn send_verification_email(&self, _to: &str, _token: &str) -> Result<(), EmailError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_signup_success() {
        let user_repo = Arc::new(MockUserRepository {
            users: Mutex::new(vec![]),
        });
        let token_repo = Arc::new(MockTokenRepository {
            tokens: Mutex::new(vec![]),
        });
        let hasher = Arc::new(MockPasswordHasher);
        let email_service = Arc::new(MockEmailService);
        let use_case =
            SignupUseCase::new(user_repo.clone(), token_repo.clone(), hasher, email_service);

        let input = SignupInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: None,
            password: "password123".to_string(),
            birth_date: None,
        };

        let user = use_case.execute(input).await.unwrap();

        assert_eq!(user.email, "john@example.com");
        assert!(!user.is_email_verified);
        assert_eq!(user.role, domain_users::models::user::UserRole::User);

        let tokens = token_repo.tokens.lock().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].user_id, user.id);
    }
}
