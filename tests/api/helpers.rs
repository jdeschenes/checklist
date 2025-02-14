use std::future::IntoFuture;
use std::sync::LazyLock;

use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool};
use secrecy::SecretBox;

use checklist::configuration::{get_configuration, DatabaseSettings};
use checklist::run;
use checklist::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
   pub address: String,
}

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber).expect("failed to initialize subscriber");
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber).expect("failed to initialize subscriber");
    };
});

async fn configure_database(configuration: &DatabaseSettings) -> PgPool {
    let maintenance_settings = DatabaseSettings {
        host: configuration.host.clone(),
        port: configuration.port,
        database: "postgres".to_string(),
        user: "postgres".to_string(),
        password: SecretBox::new(Box::new("password".to_string())),
        max_connections: configuration.max_connections,
        pool_acquire_timeout: configuration.pool_acquire_timeout,
        require_ssl: false,
    };
    let mut connection = PgConnection::connect_with(&maintenance_settings.connection_options())
        .await
        .expect("Failed to connect to postgres");

    connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}" WITH OWNER "{}";"#,
                configuration.database, configuration.user
            )
            .as_str(),
        )
        .await
        .expect("Unable to create database");

    let pool = PgPoolOptions::new()
        .max_connections(configuration.max_connections)
        .acquire_timeout(configuration.pool_acquire_timeout)
        .connect_lazy_with(configuration.connection_options());

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    pool
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);
    let mut configuration = get_configuration().expect("Unable to get configuration");
    configuration.database.database = uuid::Uuid::new_v4().to_string();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Binding listener");
    let port = listener.local_addr().unwrap().port();
    let pool = configure_database(&configuration.database).await;

    let server = run(listener, pool.clone())
        .await
        .expect("Failed to bind address");

    let _ = tokio::spawn(server.into_future());
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
    }
}
