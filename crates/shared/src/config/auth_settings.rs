use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    #[serde(default = "default_password_reset_token_expiry_hours")]
    pub password_reset_token_expiry_hours: u64,
}

fn default_password_reset_token_expiry_hours() -> u64 {
    24
}

impl Default for AuthSettings {
    fn default() -> Self {
        Self {
            password_reset_token_expiry_hours: default_password_reset_token_expiry_hours(),
        }
    }
}
