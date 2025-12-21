use eyre::{Context, Result};

use checklist::configuration::{get_configuration, AuthSettings, Environment, Settings};
use checklist::startup::{run_migrations, Application};
use checklist::telemetry::{get_subscriber, init_subscriber};
use tracing::{info, warn};

fn log_startup_configuration(configuration: &Settings) {
    let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".to_string());
    let validate_db_on_startup = configuration
        .application
        .validate_db_on_startup
        .unwrap_or(true);

    info!(environment = %environment, "Application starting");
    info!(
        app_host = %configuration.application.host,
        app_port = configuration.application.port,
        validate_db_on_startup,
        "Application configuration loaded"
    );
    info!(
        db_host = %configuration.database.host,
        db_port = configuration.database.port,
        db_user = %configuration.database.user,
        db_name = %configuration.database.database,
        db_require_ssl = configuration.database.require_ssl,
        "Database configuration loaded"
    );
    info!(
        recurring_look_ahead = ?configuration.recurring.look_ahead_duration,
        "Recurring templates configuration loaded"
    );

    match &configuration.auth {
        AuthSettings::Jwt {
            jwt_expiration_hours,
            ..
        } => {
            info!(
                auth_type = "jwt",
                jwt_expiration_hours, "Auth configuration loaded"
            );
        }
        AuthSettings::GoogleOAuth {
            jwt_expiration_hours,
            google_oauth,
            ..
        } => {
            info!(
                auth_type = "google_oauth",
                jwt_expiration_hours, "Auth configuration loaded"
            );
            info!(
                client_id = %google_oauth.client_id,
                redirect_uri = %google_oauth.redirect_uri,
                frontend_callback_url = %google_oauth.frontend_callback_url,
                auth_url = %google_oauth.auth_url,
                token_url = %google_oauth.token_url,
                userinfo_url = %google_oauth.userinfo_url,
                "Google OAuth configuration loaded"
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".to_string());
    let env_file = format!(".env.{}", environment);
    let env_result = dotenvy::from_filename(&env_file)
        .or_else(|_| dotenvy::from_filename(format!("../{}", env_file)));
    let subscriber = get_subscriber(
        "checklist".into(),
        "sqlx=error,info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber)?;
    match env_result {
        Ok(path) => info!(path = %path.display(), "Loaded environment file"),
        Err(err) => warn!(path = %env_file, error = %err, "Failed to load environment file"),
    }
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let configuration = get_configuration(environment).context("Failed to read configuration")?;
    log_startup_configuration(&configuration);
    let run_migrations_only = std::env::args()
        .skip(1)
        .any(|arg| arg == "migrate" || arg == "--migrate");
    if run_migrations_only {
        info!("Running migrations only");
        run_migrations(&configuration)
            .await
            .context("Failed to run migrations")?;
        return Ok(());
    }
    let application = Application::build(configuration)
        .await
        .context("Failed to build the server")?;
    application.run_until_stopped().await
}
