use crate::ports::TokenRepositoryError;
use crate::ports::email_service::EmailService;
use crate::ports::password_reset_token_repository::{
    PasswordResetToken, PasswordResetTokenRepository,
};
use crate::ports::user_repository::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct RequestPasswordResetUseCase {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn PasswordResetTokenRepository>,
    email_service: Arc<dyn EmailService>,
}

impl RequestPasswordResetUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_repo: Arc<dyn PasswordResetTokenRepository>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            user_repo,
            token_repo,
            email_service,
        }
    }

    pub async fn execute(&self, email: &str) -> Result<(), RequestPasswordResetError> {
        let user = self
            .user_repo
            .find_active_by_email(email)
            .await?
            .ok_or(RequestPasswordResetError::UserNotFound)?;

        // Delete any existing tokens for this user
        self.token_repo.delete_by_user_id(&user.id).await?;

        // Generate token
        let token_str = Uuid::new_v4().to_string();
        let token = PasswordResetToken {
            token: token_str.clone(),
            user_id: user.id,
            created_at: chrono::Utc::now(),
        };

        self.token_repo.create(&token).await?;

        self.email_service
            .send_password_reset_email(&user.email, &token_str, &user.first_name)
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum RequestPasswordResetError {
    UserNotFound,
    RepositoryError(String),
    EmailError(String),
}

impl From<crate::ports::user_repository::UserRepositoryError> for RequestPasswordResetError {
    fn from(err: crate::ports::user_repository::UserRepositoryError) -> Self {
        RequestPasswordResetError::RepositoryError(err.to_string())
    }
}

impl From<TokenRepositoryError> for RequestPasswordResetError {
    fn from(err: TokenRepositoryError) -> Self {
        RequestPasswordResetError::RepositoryError(err.to_string())
    }
}

impl From<crate::ports::email_service::EmailError> for RequestPasswordResetError {
    fn from(err: crate::ports::email_service::EmailError) -> Self {
        RequestPasswordResetError::EmailError(err.to_string())
    }
}

impl std::fmt::Display for RequestPasswordResetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserNotFound => write!(f, "User not found"),
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            Self::EmailError(msg) => write!(f, "Email error: {}", msg),
        }
    }
}

impl std::error::Error for RequestPasswordResetError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::TokenRepositoryError;
    use crate::ports::email_service::{EmailError, EmailService};
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
        async fn update(&self, _user: &User) -> Result<(), UserRepositoryError> {
            unimplemented!()
        }
        async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, UserRepositoryError> {
            unimplemented!()
        }
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, UserRepositoryError> {
            unimplemented!()
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
        async fn create(&self, token: &PasswordResetToken) -> Result<(), TokenRepositoryError> {
            self.tokens.lock().unwrap().push(token.clone());
            Ok(())
        }
        async fn find_by_token(
            &self,
            _token: &str,
        ) -> Result<Option<PasswordResetToken>, TokenRepositoryError> {
            unimplemented!()
        }
        async fn delete_by_token(&self, _token: &str) -> Result<(), TokenRepositoryError> {
            unimplemented!()
        }
        async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), TokenRepositoryError> {
            let mut tokens = self.tokens.lock().unwrap();
            tokens.retain(|t| t.user_id != *user_id);
            Ok(())
        }
    }

    struct MockEmailService {
        sent_emails: Mutex<Vec<(String, String, String)>>,
    }

    #[async_trait]
    impl EmailService for MockEmailService {
        async fn send_verification_email(
            &self,
            _to: &str,
            _token: &str,
            _first_name: &str,
            _last_name: &str,
        ) -> Result<(), EmailError> {
            unimplemented!()
        }

        async fn send_password_reset_email(
            &self,
            to: &str,
            token: &str,
            first_name: &str,
        ) -> Result<(), EmailError> {
            self.sent_emails.lock().unwrap().push((
                to.to_string(),
                token.to_string(),
                first_name.to_string(),
            ));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_request_password_reset_success() {
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

        let user_repo = Arc::new(MockUserRepository {
            users: Mutex::new(vec![user]),
        });
        let token_repo = Arc::new(MockTokenRepository {
            tokens: Mutex::new(vec![]),
        });
        let email_service = Arc::new(MockEmailService {
            sent_emails: Mutex::new(vec![]),
        });

        let use_case = RequestPasswordResetUseCase::new(
            user_repo.clone(),
            token_repo.clone(),
            email_service.clone(),
        );

        use_case.execute("john@example.com").await.unwrap();

        let tokens = token_repo.tokens.lock().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].user_id, user_id);

        let sent_emails = email_service.sent_emails.lock().unwrap();
        assert_eq!(sent_emails.len(), 1);
        assert_eq!(sent_emails[0].0, "john@example.com");
        assert_eq!(sent_emails[0].1, tokens[0].token);
        assert_eq!(sent_emails[0].2, "John");
    }

    #[tokio::test]
    async fn test_request_password_reset_user_not_found() {
        let user_repo = Arc::new(MockUserRepository {
            users: Mutex::new(vec![]),
        });
        let token_repo = Arc::new(MockTokenRepository {
            tokens: Mutex::new(vec![]),
        });
        let email_service = Arc::new(MockEmailService {
            sent_emails: Mutex::new(vec![]),
        });

        let use_case = RequestPasswordResetUseCase::new(user_repo, token_repo, email_service);

        let result = use_case.execute("nonexistent@example.com").await;
        assert!(matches!(
            result,
            Err(RequestPasswordResetError::UserNotFound)
        ));
    }
}
