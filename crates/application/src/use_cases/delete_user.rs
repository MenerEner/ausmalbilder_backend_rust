use crate::ports::user_repository::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteUserUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl DeleteUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DeleteUserError> {
        let user = self.user_repo.find_by_id(id).await?;

        match user {
            Some(mut user) => {
                if user.is_deleted() {
                    return Ok(());
                }
                user.delete();
                self.user_repo.update(&user).await?;
                Ok(())
            }
            None => Err(DeleteUserError::NotFound(id)),
        }
    }
}

#[derive(Debug)]
pub enum DeleteUserError {
    NotFound(Uuid),
    RepositoryError(String),
}

impl From<crate::ports::user_repository::UserRepositoryError> for DeleteUserError {
    fn from(err: crate::ports::user_repository::UserRepositoryError) -> Self {
        match err {
            crate::ports::user_repository::UserRepositoryError::NotFound(msg) => {
                Self::RepositoryError(msg)
            }
            crate::ports::user_repository::UserRepositoryError::AlreadyExists(msg) => {
                Self::RepositoryError(msg)
            }
            crate::ports::user_repository::UserRepositoryError::DatabaseError(msg) => {
                Self::RepositoryError(msg)
            }
        }
    }
}

impl std::fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(id) => write!(f, "User with ID {} not found", id),
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
        }
    }
}

impl std::error::Error for DeleteUserError {}
