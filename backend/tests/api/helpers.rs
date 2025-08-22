use std::sync::LazyLock;

use checklist::auth::JwtService;
use checklist::configuration::{get_configuration, DatabaseSettings};
use checklist::startup::{get_connection_pool, Application};
use checklist::telemetry::{get_subscriber, init_subscriber};
use secrecy::{ExposeSecret, SecretString};
use serde_json::Value as JsonValue;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use crate::golden::GoldenTest;

pub struct TestApp {
    pub address: String,
    pub golden: GoldenTest,
    pub client: reqwest::Client,
    pub db_pool: PgPool,
    pub jwt_service: JwtService,
    pub test_user_id: String,
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

async fn setup_database(configuration: &DatabaseSettings) {
    let maintenance_settings = DatabaseSettings {
        host: configuration.host.clone(),
        port: configuration.port,
        database: "postgres".to_string(),
        user: "postgres".to_string(),
        password: SecretString::new("password".to_string().into()),
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
}

async fn configure_database(configuration: &DatabaseSettings) -> PgPool {
    setup_database(configuration).await;
    let pool = get_connection_pool(configuration);
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    pool
}

pub async fn spawn_app() -> TestApp {
    spawn_app_with_config(true).await
}

async fn spawn_app_with_config(valid_app: bool) -> TestApp {
    LazyLock::force(&TRACING);

    let mut configuration = {
        let mut c = get_configuration().expect("Unable to read configuration");
        c.database.database = uuid::Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };
    let db_pool = configure_database(&configuration.database).await;
    if !valid_app {
        configuration.database.database = "INVALIDDB".to_string();
        configuration.application.validate_db_on_startup = Some(false);
    }
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to bind address");
    let address = format!("http://127.0.0.1:{}", application.port());
    tokio::spawn(application.run_until_stopped());

    // Create JWT service for testing
    let jwt_service = match &configuration.auth {
        checklist::configuration::AuthSettings::Jwt {
            jwt_secret,
            jwt_expiration_hours,
        } => JwtService::new(jwt_secret.expose_secret(), *jwt_expiration_hours),
        checklist::configuration::AuthSettings::GoogleOAuth {
            jwt_secret,
            jwt_expiration_hours,
            ..
        } => JwtService::new(jwt_secret.expose_secret(), *jwt_expiration_hours),
    };

    // Create a test user ID for authentication
    let test_user_id = Uuid::new_v4().to_string();

    // Create test user in database to satisfy foreign key constraints
    sqlx::query!(
        "INSERT INTO users (user_id, email, create_time, update_time) VALUES ($1, $2, NOW(), NOW())",
        test_user_id,
        "test@example.com"
    )
    .execute(&db_pool)
    .await
    .expect("Failed to create test user");

    TestApp {
        address,
        golden: GoldenTest::new(),
        client: reqwest::Client::new(),
        db_pool,
        jwt_service,
        test_user_id,
    }
}

pub async fn spawn_invalid_db_app() -> TestApp {
    spawn_app_with_config(false).await
}

impl TestApp {
    /// Generate JWT token for test authentication
    pub fn get_auth_header(&self) -> String {
        let token = self
            .jwt_service
            .generate_token(&self.test_user_id.to_string(), "test@example.com")
            .expect("Failed to generate test JWT token");
        format!("Bearer {}", token)
    }
    pub async fn post_todo(&self, payload: &JsonValue) -> reqwest::Response {
        self.client
            .post(format!("{}/todo", self.address))
            .header("Authorization", self.get_auth_header())
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_todo(&self, todo_name: &str) -> reqwest::Response {
        self.client
            .get(format!("{}/todo/{}", self.address, todo_name))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn update_todo(&self, todo_name: &str, payload: &JsonValue) -> reqwest::Response {
        self.client
            .put(format!("{}/todo/{}", self.address, todo_name))
            .header("Authorization", self.get_auth_header())
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_todo(&self, todo_name: &str) -> reqwest::Response {
        self.client
            .delete(format!("{}/todo/{}", self.address, todo_name))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn list_todo(&self) -> reqwest::Response {
        self.client
            .get(format!("{}/todo", self.address))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_todo_item(&self, todo_name: &str, payload: &JsonValue) -> reqwest::Response {
        self.client
            .post(format!("{}/todo/{}/item", self.address, todo_name))
            .header("Authorization", self.get_auth_header())
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_todo_item(&self, todo_name: &str, todo_item_id: &str) -> reqwest::Response {
        self.client
            .get(format!(
                "{}/todo/{}/item/{}",
                self.address, todo_name, todo_item_id
            ))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn complete_todo_item(
        &self,
        todo_name: &str,
        todo_item_id: &str,
    ) -> reqwest::Response {
        self.client
            .post(format!(
                "{}/todo/{}/item/{}/complete",
                self.address, todo_name, todo_item_id
            ))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn update_todo_item(
        &self,
        todo_name: &str,
        todo_item_id: &str,
        payload: &JsonValue,
    ) -> reqwest::Response {
        self.client
            .put(format!(
                "{}/todo/{}/item/{}",
                self.address, todo_name, todo_item_id
            ))
            .header("Authorization", self.get_auth_header())
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn list_todo_items(&self, todo_name: &str) -> reqwest::Response {
        self.client
            .get(format!("{}/todo/{}/item", self.address, todo_name))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_todo_item(&self, todo_name: &str, todo_item_id: &str) -> reqwest::Response {
        self.client
            .delete(format!(
                "{}/todo/{}/item/{}",
                self.address, todo_name, todo_item_id
            ))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_recurring_template(
        &self,
        todo_name: &str,
        payload: &JsonValue,
    ) -> reqwest::Response {
        self.client
            .post(format!("{}/todo/{}/recurring", self.address, todo_name))
            .header("Authorization", self.get_auth_header())
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn process_recurring_templates(
        &self,
        advance_duration: std::time::Duration,
    ) -> eyre::Result<()> {
        checklist::services::process_recurring_templates(&self.db_pool, advance_duration).await
    }

    pub async fn list_recurring_templates(&self, todo_name: &str) -> reqwest::Response {
        self.client
            .get(format!("{}/todo/{}/recurring", self.address, todo_name))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_recurring_template(
        &self,
        todo_name: &str,
        template_id: &str,
    ) -> reqwest::Response {
        self.client
            .get(format!(
                "{}/todo/{}/recurring/{}",
                self.address, todo_name, template_id
            ))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn update_recurring_template(
        &self,
        todo_name: &str,
        template_id: &str,
        payload: &JsonValue,
    ) -> reqwest::Response {
        self.client
            .put(format!(
                "{}/todo/{}/recurring/{}",
                self.address, todo_name, template_id
            ))
            .header("Authorization", self.get_auth_header())
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn delete_recurring_template(
        &self,
        todo_name: &str,
        template_id: &str,
    ) -> reqwest::Response {
        self.client
            .delete(format!(
                "{}/todo/{}/recurring/{}",
                self.address, todo_name, template_id
            ))
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub fn assert_response(response: &reqwest::Response, status_code: reqwest::StatusCode) {
    assert_eq!(response.status(), status_code);
    match response.headers().get("x-request-id") {
        Some(x) => {
            let y = x.to_str().expect("Expected a string in the header");
            uuid::Uuid::parse_str(y).expect("uuid expected in x-request-id");
        }
        None => {
            panic!("no x-request-id header found in response")
        }
    }
}
