use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::helpers::spawn_app;

#[derive(Deserialize)]
struct CreateResponse {
    todo_item_id: String,
}

#[tokio::test]
async fn create_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_eq!(create_todo_item_response.status(), StatusCode::OK);
    let value: serde_json::Value = create_todo_item_response
        .json()
        .await
        .expect("Failed to read json");
    test_app.golden.check_diff_json("create_todo_item", &value);
    let create_value: CreateResponse = serde_json::from_value(value).unwrap();
    let get_todo_item_response = test_app
        .get_todo_item("banana", &create_value.todo_item_id)
        .await;
    assert_eq!(get_todo_item_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let valid_payload = serde_json::json!({
        "title": "todo_item2",
    });
    let invalid_payload = serde_json::json!({
        "invalid": "todo_item2",
    });
    let test_cases = vec![
        (
            "invalid_data",
            "banana",
            invalid_payload,
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "todo not exists",
            "NOT_EXISTS",
            valid_payload,
            StatusCode::NOT_FOUND,
        ),
    ];
    for test_case in test_cases {
        let response = test_app.post_todo_item(test_case.1, &test_case.2).await;
        assert_eq!(response.status(), test_case.3, "{}", test_case.0);
    }
}

#[tokio::test]
async fn get_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_eq!(create_todo_item_response.status(), StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let get_todo_item_response = test_app
        .get_todo_item("banana", &response.todo_item_id)
        .await;
    assert_eq!(get_todo_item_response.status(), StatusCode::OK);

    let value: serde_json::Value = get_todo_item_response
        .json()
        .await
        .expect("Failed to read json");
    test_app.golden.check_diff_json("get_todo_item", &value);
}

#[tokio::test]
async fn get_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_eq!(create_todo_item_response.status(), StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let invalid_uuid = Uuid::new_v4().to_string();
    let test_cases = vec![
        (
            "NOT_EXISTS",
            response.todo_item_id.as_str(),
            StatusCode::NOT_FOUND,
        ),
        ("banana", invalid_uuid.as_str(), StatusCode::NOT_FOUND),
        ("banana", "INVALID_UUID", StatusCode::BAD_REQUEST),
    ];
    for test_case in test_cases {
        let delete_response = test_app.get_todo_item(test_case.0, test_case.1).await;
        assert_eq!(delete_response.status(), test_case.2);
    }
}

#[tokio::test]
async fn list_todo_items_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    for i in 0..50 {
        let payload: serde_json::Value =
            serde_json::from_str(&format!(r#"{{"title": "banana{i}"}}"#)).unwrap();
        let create_response = test_app.post_todo_item("banana", &payload).await;
        assert_eq!(create_response.status(), StatusCode::OK);
    }

    let list_response = test_app.list_todo_items("banana").await;
    assert_eq!(list_response.status(), StatusCode::OK);
    let expected: serde_json::Value = list_response.json().await.expect("Failed to read json");
    test_app.golden.check_diff_json("list_todo_item", &expected);
}

#[tokio::test]
async fn list_todo_items_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let test_cases = vec![("NOT_EXISTS", StatusCode::NOT_FOUND)];
    for test_case in test_cases {
        let list_response = test_app.list_todo_items(test_case.0).await;
        assert_eq!(list_response.status(), test_case.1);
    }
}

#[tokio::test]
async fn update_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_eq!(create_todo_item_response.status(), StatusCode::OK);

    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let valid_payload = serde_json::json!({
        "title": "todo_item2",
        "is_complete": true
    });
    let update_response = test_app
        .update_todo_item("banana", &response.todo_item_id, &valid_payload)
        .await;
    assert_eq!(update_response.status(), StatusCode::OK);

    let value: serde_json::Value = update_response.json().await.expect("Failed to read json");
    test_app.golden.check_diff_json("update_todo_item", &value);
}

#[tokio::test]
async fn update_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_eq!(create_todo_item_response.status(), StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let valid_payload = serde_json::json!({
        "title": "todo_item2",
        "is_complete": false,
    });
    let invalid_payload = serde_json::json!({
        "invalid": "todo_item2",
        "is_complete": false,
    });
    let invalid_uuid = Uuid::new_v4().to_string();
    let test_cases = vec![
        (
            "invalid_payload",
            "banana",
            &response.todo_item_id,
            &invalid_payload,
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "invalid_todo",
            "NOT_EXISTS",
            &response.todo_item_id,
            &valid_payload,
            StatusCode::NOT_FOUND,
        ),
        (
            "invalid_todo_item",
            "banana",
            &invalid_uuid,
            &valid_payload,
            StatusCode::NOT_FOUND,
        ),
    ];
    for test_case in test_cases {
        let update_response = test_app
            .update_todo_item(test_case.1, test_case.2, test_case.3)
            .await;
        assert_eq!(update_response.status(), test_case.4, "{}", test_case.0);
    }
}

#[tokio::test]
async fn delete_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_eq!(create_todo_item_response.status(), StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let delete_response = test_app
        .delete_todo_item("banana", &response.todo_item_id)
        .await;
    assert_eq!(delete_response.status(), StatusCode::OK);

    let get_todo_item_response = test_app
        .get_todo_item("banana", &response.todo_item_id)
        .await;
    assert_eq!(get_todo_item_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_eq!(create_todo_item_response.status(), StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let invalid_uuid = Uuid::new_v4().to_string();
    let test_cases = vec![
        (
            "NOT_EXISTS",
            response.todo_item_id.as_str(),
            StatusCode::NOT_FOUND,
        ),
        ("banana", invalid_uuid.as_str(), StatusCode::NOT_FOUND),
        ("banana", "INVALID_UUID", StatusCode::BAD_REQUEST),
    ];
    for test_case in test_cases {
        let delete_response = test_app.delete_todo_item(test_case.0, test_case.1).await;
        assert_eq!(delete_response.status(), test_case.2);
    }
}
