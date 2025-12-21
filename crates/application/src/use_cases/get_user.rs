use crate::ports::user_repository::{UserRepository, UserRepositoryError};
use domain_users::User;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<User>, GetUserError> {
        let user = self.user_repo.find_by_id(id).await?;
        Ok(user)
    }
}

#[derive(Debug)]
pub enum GetUserError {
    RepositoryError(String),
}

impl From<UserRepositoryError> for GetUserError {
    fn from(err: UserRepositoryError) -> Self {
        match err {
            UserRepositoryError::DatabaseError(msg) => Self::RepositoryError(msg),
            UserRepositoryError::AlreadyExists(msg) => Self::RepositoryError(msg), // Should not happen for find_by_id
            UserRepositoryError::NotFound(msg) => Self::RepositoryError(msg),
        }
    }
}

impl std::fmt::Display for GetUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
        }
    }
}

impl std::error::Error for GetUserError {}
