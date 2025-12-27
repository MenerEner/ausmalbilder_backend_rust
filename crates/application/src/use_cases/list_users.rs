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

    pub async fn execute(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<User>, u64), ListUsersError> {
        let (users, total) = self
            .user_repo
            .find_all_active_paginated(page, page_size)
            .await?;
        Ok((users, total))
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
