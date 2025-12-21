use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingSettings {
    #[serde(default)]
    pub format: Option<String>,
}
