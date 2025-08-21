use std::future::IntoFuture;

use eyre::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Executor;
use tokio::net::TcpListener;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use crate::configuration::{DatabaseSettings, Settings};
use crate::services::process_recurring_templates;
use crate::{run, Server};

pub struct Application {
    server: Server,
    port: u16,
    _scheduler: JobScheduler,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Application> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        tracing::info!("Binding to: {}", address);
        let listener = TcpListener::bind(address)
            .await
            .context("Binding listener")?;
        let port = listener.local_addr().unwrap().port();
        let pool = get_connection_pool(&configuration.database);
        if let Some(true) | None = configuration.application.validate_db_on_startup {
            pool.execute("SELECT 1")
                .await
                .context("Attempting to execute query on DB")?;
        }

        // Setup recurring templates scheduler
        let scheduler =
            setup_recurring_scheduler(&pool, configuration.recurring.advance_days).await?;

        let server = run(listener, pool, configuration.recurring).await?;
        Ok(Application {
            server,
            port,
            _scheduler: scheduler,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        self.server
            .into_future()
            .await
            .context("Error when serving traffic")
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(configuration.max_connections)
        .acquire_timeout(configuration.pool_acquire_timeout)
        .connect_lazy_with(configuration.connection_options())
}

async fn setup_recurring_scheduler(
    pool: &PgPool,
    advance_duration: std::time::Duration,
) -> Result<JobScheduler> {
    let scheduler = JobScheduler::new()
        .await
        .context("Failed to create job scheduler")?;

    let pool_clone = pool.clone();
    let job = Job::new_async("0 0 0 * * *", move |_uuid, _l| {
        let pool = pool_clone.clone();
        Box::pin(async move {
            info!("Starting daily recurring templates job");
            if let Err(e) = process_recurring_templates(&pool, advance_duration).await {
                error!("Recurring templates processing failed: {}", e);
            } else {
                info!("Daily recurring templates job completed successfully");
            }
        })
    })
    .context("Failed to create recurring templates job")?;

    scheduler
        .add(job)
        .await
        .context("Failed to add recurring templates job to scheduler")?;

    scheduler
        .start()
        .await
        .context("Failed to start job scheduler")?;

    info!("Recurring templates scheduler started (runs daily at midnight)");

    Ok(scheduler)
}
