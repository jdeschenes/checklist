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
    Test,
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Test => "test",
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = Report;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "test" => Ok(Self::Test),
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            _ => bail!(
                "`{}` is not a supported environment. Use either `test`, `local` or `production`",
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
    #[serde(default = "default_frontend_callback_url")]
    pub frontend_callback_url: String,
    #[serde(default = "default_auth_url")]
    pub auth_url: String,
    #[serde(default = "default_token_url")]
    pub token_url: String,
    #[serde(default = "default_userinfo_url")]
    pub userinfo_url: String,
}

fn validate_settings(settings: &Settings) -> Result<()> {
    if let AuthSettings::GoogleOAuth { google_oauth, .. } = &settings.auth {
        let mut missing = Vec::new();
        if google_oauth.client_id.is_empty() {
            missing.push("auth.google_oauth.client_id");
        }
        if google_oauth.client_secret.expose_secret().is_empty() {
            missing.push("auth.google_oauth.client_secret");
        }
        if !missing.is_empty() {
            bail!(
                "Google OAuth is enabled but the following settings are missing or placeholders: {}",
                missing.join(", ")
            );
        }
    }

    Ok(())
}

fn default_auth_url() -> String {
    "https://accounts.google.com/o/oauth2/v2/auth".to_string()
}

fn default_frontend_callback_url() -> String {
    "http://localhost:5173/auth/callback".to_string()
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

pub fn get_configuration(environment: Environment) -> Result<Settings> {
    let base_path = std::env::current_dir().context("Failed to determine the current directory")?;
    let configuration_directory = base_path.join("configuration");
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
    let settings = settings
        .try_deserialize::<Settings>()
        .context("Deserialize settings")?;
    validate_settings(&settings).context("Validate settings")?;
    Ok(settings)
}
