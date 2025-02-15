use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::helpers::spawn_app;

#[tokio::test]
async fn create_todo_works() {
    #[derive(Debug, Serialize, Deserialize)]
    struct CaseInout {
        name: String,
    }

    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();
    let test_case = (
        CaseInout {
            name: "banana".to_string(),
        },
        "banana",
    );

    let payload: serde_json::Value = serde_json::to_value(test_case.0).unwrap();
    let create_response = client
        .post(format!("{}/todo", address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(create_response.status(), StatusCode::OK);
    assert_eq!(Some(0), create_response.content_length());

    let get_response = client
        .get(format!("{}/todo/{}", address, test_case.1))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(get_response.status(), StatusCode::OK);
    let expected: serde_json::Value = get_response.json().await.expect("Failed to read json");
    test_app.golden.check_diff("get_todo", &expected);
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
        assert_eq!(
            response.status(),
            case.expected_status_code,
            "The API did not return a '{}' when the payload '{:?}'.",
            case.expected_status_code,
            case.json
        );
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
