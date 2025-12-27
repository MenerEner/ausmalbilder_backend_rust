pub mod email_service;
pub mod email_verification_token_repository;
pub mod password_hasher;
pub mod password_reset_token_repository;
pub mod user_repository;

pub use email_service::{EmailError, EmailService};
pub use email_verification_token_repository::{
    EmailVerificationToken, EmailVerificationTokenRepository,
};
pub use password_hasher::{PasswordHasher, PasswordHasherError};
pub use password_reset_token_repository::{PasswordResetToken, PasswordResetTokenRepository};
pub use user_repository::{UserRepository, UserRepositoryError};

#[derive(Debug)]
pub enum TokenRepositoryError {
    DatabaseError(String),
}

impl std::fmt::Display for TokenRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for TokenRepositoryError {}
