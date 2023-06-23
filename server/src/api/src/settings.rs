use config::{Config, ConfigError, File};
use postgres::PsqlSettings;
use serde::Deserialize;
use tracing::Level;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub log_level: LogLevel,
    pub environment: Environment,
    pub api: ApiSettings,
    pub postgres: PsqlSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiSettings {
    pub ip: String,
    pub port: u16,
    pub num_workers: Option<u32>,
    pub secret_key: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, strum::Display, strum::EnumString)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, strum::Display, strum::EnumString)]
pub enum Environment {
    Test,
    Local,
    Avskjerming,
    Td,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap()
            .as_str()
            .try_into()
            .expect("failed to parse APP_ENVIRONMENT");

        let mut builder = Config::builder()
            .add_source(File::with_name(&format!(
                "config/{}",
                environment.to_string().to_lowercase()
            )))
            .set_override("environment", environment.to_string())?;

        if environment == Environment::Avskjerming {
            let pg_password = std::env::var("PGPASSWORD").expect("failed to parse PGPASSWORD");
            builder = builder.set_override("postgres.password", pg_password)?;
        } else if environment == Environment::Td {
            let secret_key = std::env::var("SECRET_KEY").expect("failed to parse SECRET_KEY");
            let pg_password = std::env::var("PGPASSWORD").expect("failed to parse PGPASSWORD");

            builder = builder.set_override("api.secret_key", secret_key)?;
            builder = builder.set_override("postgres.password", pg_password)?;
        }

        builder.build()?.try_deserialize()
    }
}

impl ApiSettings {
    pub fn listener_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl From<LogLevel> for Level {
    fn from(v: LogLevel) -> Self {
        match v {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}
