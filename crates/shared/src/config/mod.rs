use std::net::{IpAddr, SocketAddr};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub server: ServerSettings,

    #[serde(default)]
    pub logging: LoggingSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings::default(),
            logging: LoggingSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

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

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        tracing::info!("loading config");
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
        if let Ok(port) = std::env::var("PORT") {
            if let Ok(p) = port.parse::<u16>() {
                settings.server.port = p;
            }
        }
        if let Ok(host) = std::env::var("HOST") {
            if !host.trim().is_empty() {
                settings.server.host = host;
            }
        }
        if let Ok(fmt) = std::env::var("LOG_FORMAT") {
            if !fmt.trim().is_empty() {
                settings.logging.format = Some(fmt);
            }
        }

        Ok(settings)
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        let ip: IpAddr = self.server.host.parse()?;
        Ok(SocketAddr::from((ip, self.server.port)))
    }
}
