use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::helpers::{assert_response, spawn_app};

#[derive(Deserialize)]
struct CreateResponse {
    todo_item_id: String,
}

#[tokio::test]
async fn create_todo_works() {
    #[derive(Debug, Serialize, Deserialize)]
    struct CaseInout {
        name: String,
    }

    let test_app = spawn_app().await;
    let test_case = (
        CaseInout {
            name: "banana".to_string(),
        },
        "banana",
    );

    let payload: serde_json::Value = serde_json::to_value(test_case.0).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);
    assert_eq!(Some(0), create_response.content_length());
    let get_response = test_app.get_todo(test_case.1).await;
    assert_response(&get_response, StatusCode::OK);
    let expected: serde_json::Value = get_response.json().await.expect("Failed to read json");
    test_app.golden.check_diff_json("get_todo", &expected);
}

#[tokio::test]
async fn create_todo_fails() {
    let test_app = spawn_app().await;
    let address = &test_app.address;

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
        let mut req = test_app
            .client
            .post(format!("{}/todo", address))
            .header("Authorization", test_app.get_auth_header());
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
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let get_response = test_app.get_todo("banana").await;
    assert_response(&get_response, StatusCode::OK);

    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_todo() {
    let test_app = spawn_app().await;
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let get_response = test_app.get_todo("banana").await;
    assert_response(&get_response, StatusCode::OK);

    let get_response = test_app.get_todo("DOESNOTEXIST").await;
    assert_response(&get_response, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_todo() {
    let test_app = spawn_app().await;
    for i in 0..50 {
        let payload: serde_json::Value =
            serde_json::from_str(&format!(r#"{{"name": "banana{i}"}}"#)).unwrap();
        let create_response = test_app.post_todo(&payload).await;
        assert_response(&create_response, StatusCode::OK);
    }

    let list_response = test_app.list_todo().await;
    assert_response(&list_response, StatusCode::OK);
    let expected: serde_json::Value = list_response.json().await.expect("Failed to read json");
    test_app.golden.check_diff_json("list_todo", &expected);
}

#[tokio::test]
async fn test_update_todo_works() {
    let test_app = spawn_app().await;
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let get_response = test_app.get_todo("banana").await;
    assert_response(&get_response, StatusCode::OK);

    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana2"}"#).unwrap();
    let update_response = test_app.update_todo("banana", &payload).await;
    assert_response(&update_response, StatusCode::OK);

    let get_response = test_app.get_todo("banana").await;
    assert_response(&get_response, StatusCode::NOT_FOUND);

    let get_response = test_app.get_todo("banana2").await;
    assert_response(&get_response, StatusCode::OK);
}

#[tokio::test]
async fn update_todo_fails() {
    let test_app = spawn_app().await;
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana2"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let test_cases = vec![
        (
            "TODO_NOT_EXISTS",
            "NOT_EXISTS",
            serde_json::json!({
                "name": "banana"
            }),
            StatusCode::NOT_FOUND,
        ),
        (
            "TODO_ALREADY_EXISTS",
            "banana2",
            serde_json::json!({
                "name": "banana"
            }),
            StatusCode::BAD_REQUEST,
        ),
    ];
    for test_case in test_cases {
        let update_response = test_app.update_todo(test_case.1, &test_case.2).await;
        assert_eq!(update_response.status(), test_case.3, "{}", test_case.0);
    }
}

#[tokio::test]
async fn delete_todo_works() {
    let test_app = spawn_app().await;
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let delete_response = test_app.delete_todo("banana").await;
    assert_response(&delete_response, StatusCode::OK);

    let get_response = test_app.get_todo("banana").await;
    assert_response(&get_response, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_todo_fails() {
    // Todo does not exist
    let test_app = spawn_app().await;
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let delete_response = test_app.delete_todo("NOT_EXISTS").await;
    assert_response(&delete_response, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn todo_fails_if_fatal_database_error() {
    let test_app = spawn_app().await;

    sqlx::query!("ALTER TABLE todo ADD COLUMN invalid_column TEXT not null")
        .execute(&test_app.db_pool)
        .await
        .expect("Failed to create column");

    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn delete_todo_also_deletes_items() {
    // Todo does not exist
    let test_app = spawn_app().await;
    let payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_response = test_app.post_todo(&payload).await;
    assert_response(&create_response, StatusCode::OK);

    let mut todo_item: Option<CreateResponse> = None;
    for i in 0..50 {
        let payload: serde_json::Value =
            serde_json::from_str(&format!(r#"{{"title": "banana{i}"}}"#)).unwrap();
        let create_response = test_app.post_todo_item("banana", &payload).await;
        assert_response(&create_response, StatusCode::OK);

        let response = (create_response).json().await.unwrap();
        todo_item = Some(response);
    }

    let delete_response = test_app.delete_todo("banana").await;
    assert_response(&delete_response, StatusCode::OK);

    let todo_item_response = test_app
        .get_todo_item("banana", todo_item.unwrap().todo_item_id.as_str())
        .await;
    assert_response(&todo_item_response, StatusCode::NOT_FOUND);
}
