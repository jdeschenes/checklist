use eyre::{Context, Result};

use checklist::configuration::get_configuration;
use checklist::startup::Application;
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
    let application = Application::build(configuration)
        .await
        .context("Failed to build the server")?;
    application.run_until_stopped().await
}
