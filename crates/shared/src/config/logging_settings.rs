use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingSettings {
    #[serde(default)]
    pub format: Option<String>,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self { format: None }
    }
}