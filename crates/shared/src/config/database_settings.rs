use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
}

impl std::fmt::Debug for DatabaseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let masked_url = if let Ok(mut url) = url::Url::parse(&self.url) {
            if url.password().is_some() {
                let _ = url.set_password(Some("********"));
            }
            url.to_string()
        } else {
            "********".to_string()
        };

        f.debug_struct("DatabaseSettings")
            .field("url", &masked_url)
            .finish()
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/postgres".to_string(),
        }
    }
}
