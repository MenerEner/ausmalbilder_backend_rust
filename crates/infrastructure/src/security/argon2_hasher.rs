use application::ports::{PasswordHasher, PasswordHasherError};
use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

pub struct Argon2Hasher;

#[async_trait::async_trait]
impl PasswordHasher for Argon2Hasher {
    async fn hash(&self, password: &str) -> Result<String, PasswordHasherError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| PasswordHasherError::HashError(e.to_string()))?
            .to_string())
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, PasswordHasherError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| PasswordHasherError::HashError(e.to_string()))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_argon2_hashing_and_verification() {
        let hasher = Argon2Hasher;
        let password = "my_secure_password";

        let hash = hasher.hash(password).await.expect("Hashing failed");
        assert!(hash.starts_with("$argon2id$"));

        let is_valid = hasher
            .verify(password, &hash)
            .await
            .expect("Verification failed");
        assert!(is_valid);

        let is_invalid = hasher
            .verify("wrong_password", &hash)
            .await
            .expect("Verification failed");
        assert!(!is_invalid);
    }
}
