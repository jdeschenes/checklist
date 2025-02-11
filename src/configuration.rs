use eyre::{Result, WrapErr};
use secrecy::ExposeSecret;
use std::time::Duration;

const CONFIGURATION_FILE: &str = "configuration.yaml";

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub database: String,
    pub password: secrecy::SecretBox<String>,

    #[serde(with = "humantime_serde")]
    pub pool_acquire_timeout: Duration,
    pub max_connections: u32,
}

impl DatabaseSettings {
    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database
        )
    }
}

pub fn get_configuration() -> Result<Settings> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            CONFIGURATION_FILE,
            config::FileFormat::Yaml,
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .context("Building settings")?;
    settings
        .try_deserialize::<Settings>()
        .context("Deserialize settings")
}
