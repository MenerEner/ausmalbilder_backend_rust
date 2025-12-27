use async_trait::async_trait;

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_verification_email(
        &self,
        to: &str,
        token: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<(), EmailError>;
}

#[derive(Debug)]
pub enum EmailError {
    SendError(String),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendError(msg) => write!(f, "Email send error: {}", msg),
        }
    }
}

impl std::error::Error for EmailError {}
