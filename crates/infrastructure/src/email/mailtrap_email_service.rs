use application::ports::email_service::{EmailError, EmailService};
use async_trait::async_trait;
use reqwest::Client;
use serde::Serialize;
use shared::config::MailtrapSettings;
use std::collections::HashMap;

pub struct MailtrapEmailService {
    client: Client,
    settings: MailtrapSettings,
}

impl MailtrapEmailService {
    pub fn new(settings: MailtrapSettings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }
}

#[derive(Serialize)]
struct MailtrapRecipient {
    email: String,
}

#[derive(Serialize)]
struct MailtrapSender {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct MailtrapSendRequest {
    from: MailtrapSender,
    to: Vec<MailtrapRecipient>,
    template_uuid: String,
    template_variables: HashMap<String, String>,
}

#[async_trait]
impl EmailService for MailtrapEmailService {
    async fn send_verification_email(
        &self,
        to: &str,
        token: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<(), EmailError> {
        let mut template_variables = HashMap::new();
        // The user asked to remove non-existing fields instead of dummy values.
        // We now include the user's name as they are available during signup.
        template_variables.insert("first_name".to_string(), first_name.to_string());
        template_variables.insert("name".to_string(), format!("{} {}", first_name, last_name));

        let confirmation_link = format!("{}?id={}", self.settings.verification_base_url, token);
        template_variables.insert("confirmation_link".to_string(), confirmation_link);

        let request_body = MailtrapSendRequest {
            from: MailtrapSender {
                email: self.settings.sender_email.clone(),
                name: self.settings.sender_name.clone(),
            },
            to: vec![MailtrapRecipient {
                email: to.to_string(),
            }],
            template_uuid: self.settings.verification_template_uuid.clone(),
            template_variables,
        };

        let response = self
            .client
            .post("https://send.api.mailtrap.io/api/send")
            .header(
                "Authorization",
                format!("Bearer {}", self.settings.api_token),
            )
            .json(&request_body)
            .send()
            .await
            .map_err(|e| EmailError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(EmailError::SendError(format!(
                "Mailtrap API returned error: {}",
                error_text
            )));
        }

        Ok(())
    }

    async fn send_password_reset_email(
        &self,
        to: &str,
        token: &str,
        first_name: &str,
    ) -> Result<(), EmailError> {
        let mut template_variables = HashMap::new();
        template_variables.insert("first_name".to_string(), first_name.to_string());

        let password_reset_link =
            format!("{}?token={}", self.settings.password_reset_base_url, token);
        template_variables.insert("password_reset_link".to_string(), password_reset_link);

        let request_body = MailtrapSendRequest {
            from: MailtrapSender {
                email: self.settings.sender_email.clone(),
                name: self.settings.sender_name.clone(),
            },
            to: vec![MailtrapRecipient {
                email: to.to_string(),
            }],
            template_uuid: self.settings.password_reset_template_uuid.clone(),
            template_variables,
        };

        let response = self
            .client
            .post("https://send.api.mailtrap.io/api/send")
            .header(
                "Authorization",
                format!("Bearer {}", self.settings.api_token),
            )
            .json(&request_body)
            .send()
            .await
            .map_err(|e| EmailError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(EmailError::SendError(format!(
                "Mailtrap API returned error: {}",
                error_text
            )));
        }

        Ok(())
    }
}
