use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MailtrapSettings {
    pub api_token: String,
    pub sender_email: String,
    pub sender_name: String,
    pub verification_template_uuid: String,
    pub verification_base_url: String,
    pub password_reset_template_uuid: String,
    pub password_reset_base_url: String,
}

impl Default for MailtrapSettings {
    fn default() -> Self {
        Self {
            api_token: "YOUR_API_TOKEN".to_string(),
            sender_email: "hello@bendyk.com".to_string(),
            sender_name: "Mailtrap Test".to_string(),
            verification_template_uuid: "fb2a7961-7820-4627-a15f-844eb32185d2".to_string(),
            verification_base_url: "http://localhost:3030/user/mailverfication.html".to_string(),
            password_reset_template_uuid: "23b0bba6-f54e-45fa-85a5-f219c89df3e0".to_string(),
            password_reset_base_url: "http://localhost:3030/user/reset-password.html".to_string(),
        }
    }
}
