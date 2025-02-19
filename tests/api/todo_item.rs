use reqwest::StatusCode;
use serde::Deserialize;

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

    // Invalid data
    // Todo doesn't exist

    unimplemented!()
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
    // Todo doesn't exist
    // TODO ITEM doesn't exist
    // TODO ITEM is not a uuid
    unimplemented!()
}

#[tokio::test]
async fn list_todo_items_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);
    unimplemented!()
}

#[tokio::test]
async fn list_todo_items_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);
    // TODO doesn't exist
    unimplemented!()
}

#[tokio::test]
async fn update_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);
    unimplemented!()
}

#[tokio::test]
async fn update_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);
    // TODO doesn't exist
    // TODO Item doesn't exist
    // TODO item is not a uuid
    unimplemented!()
}

#[tokio::test]
async fn delete_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);
    unimplemented!()
}

#[tokio::test]
async fn delete_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value = serde_json::from_str(r#"{"name": "banana"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_eq!(create_todo_response.status(), StatusCode::OK);
    // TODO doesn't exist
    // TODO Item doesn't exist
    // TODO item is not a uuid
    unimplemented!()
}
