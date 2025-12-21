use crate::ports::user_repository::{UserRepository, UserRepositoryError};
use domain_users::User;
use std::sync::Arc;

pub struct ListUsersUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl ListUsersUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self) -> Result<Vec<User>, ListUsersError> {
        let users = self.user_repo.find_all_active().await?;
        Ok(users)
    }
}

#[derive(Debug)]
pub enum ListUsersError {
    RepositoryError(String),
}

impl From<UserRepositoryError> for ListUsersError {
    fn from(err: UserRepositoryError) -> Self {
        match err {
            UserRepositoryError::DatabaseError(msg) => Self::RepositoryError(msg),
            UserRepositoryError::AlreadyExists(msg) => Self::RepositoryError(msg),
            UserRepositoryError::NotFound(msg) => Self::RepositoryError(msg),
        }
    }
}

impl std::fmt::Display for ListUsersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
        }
    }
}

impl std::error::Error for ListUsersError {}
