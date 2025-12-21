#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, PasswordHasherError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, PasswordHasherError>;
}

#[derive(Debug)]
pub enum PasswordHasherError {
    HashError(String),
}

impl std::fmt::Display for PasswordHasherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashError(msg) => write!(f, "Hash error: {}", msg),
        }
    }
}

impl std::error::Error for PasswordHasherError {}
