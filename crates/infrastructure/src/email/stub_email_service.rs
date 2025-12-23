use application::ports::email_service::{EmailError, EmailService};
use async_trait::async_trait;

pub struct StubEmailService;

#[async_trait]
impl EmailService for StubEmailService {
    async fn send_verification_email(&self, to: &str, token: &str) -> Result<(), EmailError> {
        tracing::info!(
            "Stub: Sending verification email to {} with token {}",
            to,
            token
        );
        Ok(())
    }
}
