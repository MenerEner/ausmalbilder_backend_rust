use crate::config::auth_settings::AuthSettings;
use crate::config::database_settings::DatabaseSettings;
use crate::config::logging_settings::LoggingSettings;
use crate::config::mailtrap_settings::MailtrapSettings;
use crate::config::server_settings::ServerSettings;
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub env: String,

    #[serde(default)]
    pub server: ServerSettings,

    #[serde(default)]
    pub logging: LoggingSettings,

    #[serde(default)]
    pub database: DatabaseSettings,

    #[serde(default)]
    pub mailtrap: MailtrapSettings,

    #[serde(default)]
    pub auth: AuthSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            env: "prod".to_string(),
            server: ServerSettings::default(),
            logging: LoggingSettings::default(),
            database: DatabaseSettings::default(),
            mailtrap: MailtrapSettings::default(),
            auth: AuthSettings::default(),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let defaults = Settings::default();

        let builder = config::Config::builder()
            .set_default("server.host", defaults.server.host)?
            .set_default("server.port", defaults.server.port as i64)?
            .add_source(config::File::with_name("config.yml").required(true))
            .add_source(
                config::Environment::with_prefix("API")
                    .separator("__")
                    .try_parsing(true),
            );

        let mut settings: Settings = builder.build()?.try_deserialize()?;

        // Railway-friendly overrides
        if let Some(url) = std::env::var("DATABASE_URL")
            .ok()
            .filter(|u| !u.trim().is_empty())
        {
            settings.database.url = url;
        }
        if let Some(p) = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
        {
            settings.server.port = p;
        }
        if let Some(host) = std::env::var("HOST").ok().filter(|h| !h.trim().is_empty()) {
            settings.server.host = host;
        }
        if let Some(fmt) = std::env::var("LOG_FORMAT")
            .ok()
            .filter(|f| !f.trim().is_empty())
        {
            settings.logging.format = Some(fmt);
        }

        if let Some(token) = std::env::var("MAILTRAP_API_TOKEN")
            .ok()
            .filter(|t| !t.trim().is_empty())
        {
            settings.mailtrap.api_token = token;
        }

        if let Some(base_url) = std::env::var("MAILTRAP_VERIFICATION_BASE_URL")
            .ok()
            .filter(|u| !u.trim().is_empty())
        {
            settings.mailtrap.verification_base_url = base_url;
        }

        Ok(settings)
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        let ip: IpAddr = self.server.host.parse()?;
        Ok(SocketAddr::from((ip, self.server.port)))
    }
}
