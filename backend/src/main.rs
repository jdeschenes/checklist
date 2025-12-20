use eyre::{Context, Result};

use checklist::configuration::get_configuration;
use checklist::startup::{run_migrations, Application};
use checklist::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = get_subscriber(
        "checklist".into(),
        "sqlx=error,info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber)?;
    let configuration = get_configuration().context("Failed to read configuration")?;
    let run_migrations_only = std::env::args()
        .skip(1)
        .any(|arg| arg == "migrate" || arg == "--migrate");
    if run_migrations_only {
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
