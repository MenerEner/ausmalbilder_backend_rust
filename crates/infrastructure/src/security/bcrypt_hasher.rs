use application::ports::{PasswordHasher, PasswordHasherError};
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct BcryptHasher;

#[async_trait::async_trait]
impl PasswordHasher for BcryptHasher {
    async fn hash(&self, password: &str) -> Result<String, PasswordHasherError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| PasswordHasherError::HashError(e.to_string()))
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, PasswordHasherError> {
        verify(password, hash)
            .map_err(|e| PasswordHasherError::HashError(e.to_string()))
    }
}
