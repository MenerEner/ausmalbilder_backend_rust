use async_trait::async_trait;
use domain_users::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;
}

#[derive(Debug)]
pub enum UserRepositoryError {
    DatabaseError(String),
    AlreadyExists(String),
}

impl std::fmt::Display for UserRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::AlreadyExists(msg) => write!(f, "User already exists: {}", msg),
        }
    }
}

impl std::error::Error for UserRepositoryError {}
