use eyre::{bail, Report, Result, WrapErr};
use secrecy::ExposeSecret;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::time::Duration;

const CONFIGURATION_FILE: &str = "base.yaml";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub recurring: RecurringSettings,
    pub auth: AuthSettings,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = Report;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            _ => bail!(
                "`{}` is not a supported environment. Use either `local` or `production`",
                value
            ),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub validate_db_on_startup: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RecurringSettings {
    /// Duration in advance to create recurring todo items
    #[serde(with = "humantime_serde")]
    pub look_ahead_duration: Duration,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub database: String,
    pub password: secrecy::SecretString,

    #[serde(with = "humantime_serde")]
    pub pool_acquire_timeout: Duration,
    pub max_connections: u32,
    pub require_ssl: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum AuthSettings {
    #[serde(rename = "jwt")]
    Jwt {
        jwt_secret: secrecy::SecretString,
        jwt_expiration_hours: u64,
    },
    #[serde(rename = "google_oauth")]
    GoogleOAuth {
        jwt_secret: secrecy::SecretString,
        jwt_expiration_hours: u64,
        google_oauth: GoogleOAuthSettings,
    },
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GoogleOAuthSettings {
    pub client_id: String,
    pub client_secret: secrecy::SecretString,
    pub redirect_uri: String,
    #[serde(default = "default_auth_url")]
    pub auth_url: String,
    #[serde(default = "default_token_url")]
    pub token_url: String,
    #[serde(default = "default_userinfo_url")]
    pub userinfo_url: String,
}

fn default_auth_url() -> String {
    "https://accounts.google.com/o/oauth2/v2/auth".to_string()
}

fn default_token_url() -> String {
    "https://www.googleapis.com/oauth2/v3/token".to_string()
}

fn default_userinfo_url() -> String {
    "https://www.googleapis.com/oauth2/v2/userinfo".to_string()
}

impl DatabaseSettings {
    pub fn connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.user)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database)
    }
}

pub fn get_configuration() -> Result<Settings> {
    let base_path = std::env::current_dir().context("Failed to determine the current directory")?;
    let configuration_directory = base_path.join("configuration");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_file = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join(CONFIGURATION_FILE),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_file),
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
