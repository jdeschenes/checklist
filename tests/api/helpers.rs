use std::sync::LazyLock;

use checklist::configuration::{get_configuration, DatabaseSettings};
use checklist::startup::{get_connection_pool, Application};
use checklist::telemetry::{get_subscriber, init_subscriber};
use secrecy::SecretBox;
use serde_json::Value as JsonValue;
use sqlx::{Connection, Executor, PgConnection};

use crate::golden::GoldenTest;

pub struct TestApp {
    pub address: String,
    pub golden: GoldenTest,
    pub client: reqwest::Client,
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

async fn configure_database(configuration: &DatabaseSettings) -> () {
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

    let pool = get_connection_pool(configuration);

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("Unable to read configuration");
        c.database.database = uuid::Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };
    configure_database(&configuration.database).await;

    let application = Application::build(configuration)
        .await
        .expect("Failed to bind address");
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());
    TestApp {
        address,
        golden: GoldenTest::new(),
        client: reqwest::Client::new(),
    }
}

impl TestApp {
    pub async fn post_todo(&self, payload: &JsonValue) -> reqwest::Response {
        self.client
            .post(format!("{}/todo", self.address))
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_todo(&self, todo_name: &str) -> reqwest::Response {
        self.client
            .get(format!("{}/todo/{}", self.address, todo_name))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn list_todo(&self) -> reqwest::Response {
        self.client
            .get(format!("{}/todo", self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }
}
