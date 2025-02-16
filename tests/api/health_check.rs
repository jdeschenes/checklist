use reqwest::StatusCode;

use crate::helpers::{spawn_app, spawn_invalid_db_app};

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
async fn health_check_fails_no_db() {
    let test_app = spawn_invalid_db_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
