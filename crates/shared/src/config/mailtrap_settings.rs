use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MailtrapSettings {
    pub api_token: String,
    pub sender_email: String,
    pub sender_name: String,
    pub verification_template_uuid: String,
    pub verification_base_url: String,
}

impl Default for MailtrapSettings {
    fn default() -> Self {
        Self {
            api_token: "YOUR_API_TOKEN".to_string(),
            sender_email: "hello@bendyk.com".to_string(),
            sender_name: "Mailtrap Test".to_string(),
            verification_template_uuid: "fb2a7961-7820-4627-a15f-844eb32185d2".to_string(),
            verification_base_url: "http://localhost:3030/user/mailverfication.html".to_string(),
        }
    }
}
