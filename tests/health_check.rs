use std::future::IntoFuture;
use std::sync::LazyLock;

use reqwest::StatusCode;
use secrecy::SecretBox;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool};

use checklist::configuration::{get_configuration, DatabaseSettings};
use checklist::run;
use checklist::telemetry::{get_subscriber, init_subscriber};

struct TestApp {
    address: String,
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

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_todo_works() {
    #[derive(Debug, Serialize, Deserialize)]
    struct CaseInout {
        name: String,
    }

    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();
    let test_cases = vec![
        (
            CaseInout {
                name: "banana".to_string(),
            },
            "banana",
        ),
        (
            CaseInout {
                name: "  banana2  ".to_string(),
            },
            "banana2",
        ),
    ];

    for case in test_cases {
        let payload: serde_json::Value = serde_json::to_value(case.0).unwrap();
        let create_response = client
            .post(format!("{}/todo", address))
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(create_response.status(), StatusCode::OK);

        let get_response = client
            .get(format!("{}/todo/{}", address, case.1))
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(get_response.status(), StatusCode::OK);
    }
}

#[tokio::test]
async fn create_todo_fails() {
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    struct FailCall {
        expected_status_code: StatusCode,
        json: Option<JsonValue>,
    }
    let cases = vec![
        FailCall {
            json: None,
            expected_status_code: StatusCode::UNSUPPORTED_MEDIA_TYPE,
        },
        FailCall {
            json: Some("BANANA".into()),
            expected_status_code: StatusCode::UNPROCESSABLE_ENTITY,
        },
        FailCall {
            json: Some(serde_json::from_str(r#"{"name": ""}"#).unwrap()),
            expected_status_code: StatusCode::BAD_REQUEST,
        },
        FailCall {
            json: Some(
                serde_json::from_str(r#"{"name": "12345678901234567890123456789"}"#).unwrap(),
            ),
            expected_status_code: StatusCode::BAD_REQUEST,
        },
    ];
    // Forgot to include a body
    for case in cases {
        let mut req = client.post(format!("{}/todo", address));
        if let Some(ref json) = case.json {
            req = req.json(json);
        }
        let response = req.send().await.expect("Failed to execute request");
        assert_eq!(response.status(), case.expected_status_code, "The API did not return a '{}' when the payload '{:?}'.", case.expected_status_code, case.json);
    }
}

#[tokio::test]
async fn create_todo_fails_if_already_exists() {
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = client
        .post(format!("{}/todo", address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(create_response.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/todo/{}", address, "banana"))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(get_response.status(), StatusCode::OK);

    let create_response = client
        .post(format!("{}/todo", address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(
        create_response.status(),
        StatusCode::BAD_REQUEST,
        "Fails if it already exists"
    );
}

#[tokio::test]
async fn get_todo() {
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = client
        .post(format!("{}/todo", address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(create_response.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/todo/{}", address, "banana"))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(get_response.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/todo/{}", address, "DOESNOTEXIST"))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

pub async fn configure_database(configuration: &DatabaseSettings) -> PgPool {
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

async fn spawn_app() -> TestApp {
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
