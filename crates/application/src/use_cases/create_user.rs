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
        if self.user_repo.find_by_email(&input.email).await?.is_some() {
            return Err(CreateUserError::AlreadyExists(input.email));
        }

        let password_hash = self.password_hasher.hash(&input.password).await.map_err(|e| {
            CreateUserError::InternalError(format!("Failed to hash password: {}", e))
        })?;

        let user = User::new(
            Uuid::new_v4(),
            input.name,
            input.email,
            input.phone_number,
            password_hash,
        );

        self.user_repo.create(&user).await?;

        Ok(user)
    }
}

pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
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
