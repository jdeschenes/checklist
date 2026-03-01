use reqwest::StatusCode;
use serde::Deserialize;
use time::Date;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::helpers::{assert_response, spawn_app};

#[derive(Deserialize)]
struct CreateResponse {
    todo_item_id: String,
    due_date: Date,
}

#[derive(Deserialize)]
struct ListResponse {
    items: Vec<serde_json::Value>,
}

#[tokio::test]
async fn create_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let value: serde_json::Value = create_todo_item_response
        .json()
        .await
        .expect("Failed to read json");
    test_app.golden.check_diff_json("create_todo_item", &value);
    let create_value: CreateResponse = serde_json::from_value(value).unwrap();
    let get_todo_item_response = test_app
        .get_todo_item("banana", &create_value.todo_item_id)
        .await;
    assert_response(&get_todo_item_response, StatusCode::OK);
}

#[tokio::test]
async fn create_todo_item_with_due_date() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item",
        "due_date": "2030-10-01"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let value: serde_json::Value = create_todo_item_response
        .json()
        .await
        .expect("Failed to read json");
    let create_value: CreateResponse = serde_json::from_value(value).unwrap();
    assert_eq!(create_value.due_date.year(), 2030);
}

#[tokio::test]
async fn create_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

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

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let get_todo_item_response = test_app
        .get_todo_item("banana", &response.todo_item_id)
        .await;
    assert_response(&get_todo_item_response, StatusCode::OK);

    let value: serde_json::Value = get_todo_item_response
        .json()
        .await
        .expect("Failed to read json");
    test_app.golden.check_diff_json("get_todo_item", &value);
}

#[tokio::test]
async fn get_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let invalid_uuid = Uuid::new_v4().to_string();
    let test_cases = vec![
        (
            "todo_no_exists",
            "NOT_EXISTS",
            response.todo_item_id.as_str(),
            StatusCode::NOT_FOUND,
        ),
        (
            "todo_item_no_exists",
            "banana",
            invalid_uuid.as_str(),
            StatusCode::NOT_FOUND,
        ),
        (
            "todo_item_not_uuid",
            "banana",
            "INVALID_UUID",
            StatusCode::BAD_REQUEST,
        ),
    ];
    for test_case in test_cases {
        let delete_response = test_app.get_todo_item(test_case.1, test_case.2).await;
        assert_eq!(delete_response.status(), test_case.3, "{}", test_case.0);
    }
}

#[tokio::test]
async fn list_todo_items_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    for i in 0..50 {
        let payload: serde_json::Value =
            serde_json::from_str(&format!(r#"{{"title": "banana{i}"}}"#)).unwrap();
        let create_response = test_app.post_todo_item("banana", &payload).await;
        assert_response(&create_response, StatusCode::OK);
    }

    let list_response = test_app.list_todo_items("banana").await;
    assert_response(&list_response, StatusCode::OK);
    let expected: serde_json::Value = list_response.json().await.expect("Failed to read json");
    test_app.golden.check_diff_json("list_todo_item", &expected);
}

#[tokio::test]
async fn list_todo_items_only_show_incomplete() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let mut todo_item: Option<CreateResponse> = None;
    for i in 0..50 {
        let payload: serde_json::Value =
            serde_json::from_str(&format!(r#"{{"title": "banana{i}"}}"#)).unwrap();
        let create_response = test_app.post_todo_item("banana", &payload).await;
        assert_response(&create_response, StatusCode::OK);

        let response = (create_response).json().await.unwrap();
        todo_item = Some(response);
    }

    let complete_response = test_app
        .complete_todo_item("banana", &todo_item.unwrap().todo_item_id)
        .await;
    assert_response(&complete_response, StatusCode::OK);

    let list_response = test_app.list_todo_items("banana").await;
    assert_response(&list_response, StatusCode::OK);

    let response: ListResponse = list_response.json().await.expect("Error parsing json");
    assert_eq!(response.items.len(), 49);
}

#[tokio::test]
async fn list_todo_items_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let test_cases = vec![("NOT_EXISTS", StatusCode::NOT_FOUND)];
    for test_case in test_cases {
        let list_response = test_app.list_todo_items(test_case.0).await;
        assert_eq!(list_response.status(), test_case.1, "{}", test_case.0);
    }
}

#[tokio::test]
async fn list_today_items_works_for_today_and_overdue_only() {
    let test_app = spawn_app().await;

    let todo_a: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let todo_b: serde_json::Value =
        serde_json::from_str(r#"{"name": "apple", "visibility": "private"}"#).unwrap();
    assert_response(&test_app.post_todo(&todo_a).await, StatusCode::OK);
    assert_response(&test_app.post_todo(&todo_b).await, StatusCode::OK);

    let today = OffsetDateTime::now_utc().date();
    let overdue = today.previous_day().expect("expected a previous day");
    let future = today.next_day().expect("expected a next day");

    let overdue_payload = serde_json::json!({
        "title": "overdue",
        "due_date": overdue.to_string(),
    });
    let today_payload = serde_json::json!({
        "title": "today",
        "due_date": today.to_string(),
    });
    let future_payload = serde_json::json!({
        "title": "future",
        "due_date": future.to_string(),
    });

    assert_response(
        &test_app.post_todo_item("banana", &overdue_payload).await,
        StatusCode::OK,
    );
    assert_response(
        &test_app.post_todo_item("apple", &today_payload).await,
        StatusCode::OK,
    );
    assert_response(
        &test_app.post_todo_item("banana", &future_payload).await,
        StatusCode::OK,
    );

    let response = test_app.list_today_items().await;
    assert_response(&response, StatusCode::OK);
    let value: serde_json::Value = response.json().await.expect("Failed to parse response");
    test_app.golden.check_diff_json("list_today_items", &value);
    let items = value["items"].as_array().expect("items should be an array");

    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|item| item["is_complete"] == false));
    assert!(items
        .iter()
        .all(|item| item["todo_name"] == "banana" || item["todo_name"] == "apple"));
    assert!(items.iter().all(|item| item["title"] != "future"));
}

#[tokio::test]
async fn list_today_items_excludes_completed_items() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    assert_response(&test_app.post_todo(&todo_payload).await, StatusCode::OK);

    let today = OffsetDateTime::now_utc().date();
    let item_payload = serde_json::json!({
        "title": "to-complete",
        "due_date": today.to_string(),
    });
    let created = test_app.post_todo_item("banana", &item_payload).await;
    assert_response(&created, StatusCode::OK);
    let created_json: serde_json::Value = created.json().await.expect("Failed to parse response");
    let item_id = created_json["todo_item_id"]
        .as_str()
        .expect("todo_item_id should be present")
        .to_string();

    assert_response(
        &test_app.complete_todo_item("banana", &item_id).await,
        StatusCode::OK,
    );

    let response = test_app.list_today_items().await;
    assert_response(&response, StatusCode::OK);
    let value: serde_json::Value = response.json().await.expect("Failed to parse response");
    let items = value["items"].as_array().expect("items should be an array");
    assert!(items.is_empty());
}

#[tokio::test]
async fn list_today_items_respects_private_and_public_visibility() {
    let test_app = spawn_app().await;
    let other_user_id = test_app.create_user("other@example.com").await;
    let other_auth = test_app.get_auth_header_for_user(other_user_id, "other@example.com");
    let today = OffsetDateTime::now_utc().date();

    let own_todo = serde_json::json!({ "name": "own-private", "visibility": "private" });
    let own_create = test_app.post_todo(&own_todo).await;
    assert_response(&own_create, StatusCode::OK);
    let own_item = serde_json::json!({ "title": "own-task", "due_date": today.to_string() });
    let own_item_create = test_app.post_todo_item("own-private", &own_item).await;
    assert_response(&own_item_create, StatusCode::OK);

    let other_private_todo =
        serde_json::json!({ "name": "other-private", "visibility": "private" });
    let other_private_create = test_app
        .client
        .post(format!("{}/todo", test_app.address))
        .header("Authorization", &other_auth)
        .json(&other_private_todo)
        .send()
        .await
        .expect("Failed to create other private todo");
    assert_response(&other_private_create, StatusCode::OK);
    let other_private_item =
        serde_json::json!({ "title": "hidden-task", "due_date": today.to_string() });
    let other_private_item_create = test_app
        .client
        .post(format!(
            "{}/todo/{}/item",
            test_app.address, "other-private"
        ))
        .header("Authorization", &other_auth)
        .json(&other_private_item)
        .send()
        .await
        .expect("Failed to create private item for other user");
    assert_response(&other_private_item_create, StatusCode::OK);

    let other_public_todo = serde_json::json!({ "name": "other-public", "visibility": "public" });
    let other_public_create = test_app
        .client
        .post(format!("{}/todo", test_app.address))
        .header("Authorization", &other_auth)
        .json(&other_public_todo)
        .send()
        .await
        .expect("Failed to create other public todo");
    assert_response(&other_public_create, StatusCode::OK);
    let other_public_item =
        serde_json::json!({ "title": "public-task", "due_date": today.to_string() });
    let other_public_item_create = test_app
        .client
        .post(format!("{}/todo/{}/item", test_app.address, "other-public"))
        .header("Authorization", &other_auth)
        .json(&other_public_item)
        .send()
        .await
        .expect("Failed to create public item for other user");
    assert_response(&other_public_item_create, StatusCode::OK);

    let list_response = test_app.list_today_items().await;
    assert_response(&list_response, StatusCode::OK);
    let value: serde_json::Value = list_response
        .json()
        .await
        .expect("Failed to parse list_today_items response");
    let items = value["items"].as_array().expect("items should be an array");

    assert!(items
        .iter()
        .any(|item| item["todo_name"] == "own-private" && item["title"] == "own-task"));
    assert!(items
        .iter()
        .any(|item| item["todo_name"] == "other-public" && item["title"] == "public-task"));
    assert!(!items
        .iter()
        .any(|item| item["todo_name"] == "other-private" && item["title"] == "hidden-task"));
}

#[tokio::test]
async fn update_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);

    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let valid_payload = serde_json::json!({
        "title": "todo_item2",
        "is_complete": true,
        "due_date": "2200-10-02"
    });
    let update_response = test_app
        .update_todo_item("banana", &response.todo_item_id, &valid_payload)
        .await;
    assert_response(&update_response, StatusCode::OK);

    let value: serde_json::Value = update_response.json().await.expect("Failed to read json");
    test_app.golden.check_diff_json("update_todo_item", &value);
    let create_value: CreateResponse = serde_json::from_value(value).unwrap();
    assert_eq!(create_value.due_date.year(), 2200);
}

#[tokio::test]
async fn update_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let valid_payload = serde_json::json!({
        "title": "todo_item2",
        "is_complete": false,
        "due_date": "2200-10-02"
    });
    let invalid_payload = serde_json::json!({
        "invalid": "todo_item2",
        "is_complete": false,
        "due_date": "2200-10-02"
    });
    let invalid_uuid = Uuid::new_v4().to_string();
    let test_cases = vec![
        (
            "invalid_payload",
            "banana",
            response.todo_item_id.as_str(),
            &invalid_payload,
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "invalid_todo",
            "NOT_EXISTS",
            response.todo_item_id.as_str(),
            &valid_payload,
            StatusCode::NOT_FOUND,
        ),
        (
            "invalid_todo_item",
            "banana",
            invalid_uuid.as_str(),
            &valid_payload,
            StatusCode::NOT_FOUND,
        ),
        (
            "invalid_todo_item_uuid",
            "banana",
            "INVALID_UUID",
            &valid_payload,
            StatusCode::BAD_REQUEST,
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
async fn update_todo_item_fails_if_already_complete() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);

    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let complete_todo_item_response = test_app
        .complete_todo_item("banana", &response.todo_item_id)
        .await;
    assert_response(&complete_todo_item_response, StatusCode::OK);

    let valid_payload = serde_json::json!({
        "title": "todo_item2",
        "is_complete": true,
        "due_date": "2200-10-02"
    });
    let update_response = test_app
        .update_todo_item("banana", &response.todo_item_id, &valid_payload)
        .await;
    assert_response(&update_response, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn complete_todo_item_works() {
    // CHECK THAT THE COMPLETE TIME IS SET
    // Implement datetime parsing
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let complete_todo_item_response = test_app
        .complete_todo_item("banana", &response.todo_item_id)
        .await;
    assert_response(&complete_todo_item_response, StatusCode::OK);

    let value: serde_json::Value = complete_todo_item_response
        .json()
        .await
        .expect("Failed to read json");
    test_app
        .golden
        .check_diff_json("complete_todo_item", &value);
}

#[tokio::test]
async fn complete_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);

    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let invalid_uuid = Uuid::new_v4().to_string();
    let test_cases = vec![
        (
            "todo_no_exists",
            "NOT_EXISTS",
            response.todo_item_id.as_str(),
            StatusCode::NOT_FOUND,
        ),
        (
            "todo_item_no_exists",
            "banana",
            invalid_uuid.as_str(),
            StatusCode::NOT_FOUND,
        ),
        (
            "todo_item_not_uuid",
            "banana",
            "NOT_UUID",
            StatusCode::BAD_REQUEST,
        ),
    ];
    for test_case in test_cases {
        let complete_response = test_app.complete_todo_item(test_case.1, test_case.2).await;
        assert_eq!(complete_response.status(), test_case.3, "{}", test_case.0);
    }
}

#[tokio::test]
async fn complete_todo_item_fails_if_already_complete() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let complete_todo_item_response = test_app
        .complete_todo_item("banana", &response.todo_item_id)
        .await;
    assert_response(&complete_todo_item_response, StatusCode::OK);

    let complete_todo_item_response = test_app
        .complete_todo_item("banana", &response.todo_item_id)
        .await;
    assert_response(&complete_todo_item_response, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn delete_todo_item_works() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let delete_response = test_app
        .delete_todo_item("banana", &response.todo_item_id)
        .await;
    assert_response(&delete_response, StatusCode::OK);

    let get_todo_item_response = test_app
        .get_todo_item("banana", &response.todo_item_id)
        .await;
    assert_response(&get_todo_item_response, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_todo_item_fails() {
    let test_app = spawn_app().await;

    let todo_payload: serde_json::Value =
        serde_json::from_str(r#"{"name": "banana", "visibility": "private"}"#).unwrap();
    let create_todo_response = test_app.post_todo(&todo_payload).await;
    assert_response(&create_todo_response, StatusCode::OK);

    let todo_item_payload = serde_json::from_str(
        r#"{
        "title": "todo_item"
    }"#,
    )
    .unwrap();
    let create_todo_item_response = test_app.post_todo_item("banana", &todo_item_payload).await;
    assert_response(&create_todo_item_response, StatusCode::OK);
    let response: CreateResponse = create_todo_item_response
        .json()
        .await
        .expect("Error parsing json");

    let invalid_uuid = Uuid::new_v4().to_string();
    let test_cases = vec![
        (
            "todo_no_exists",
            "NOT_EXISTS",
            response.todo_item_id.as_str(),
            StatusCode::NOT_FOUND,
        ),
        (
            "todo_item_no_exists",
            "banana",
            invalid_uuid.as_str(),
            StatusCode::NOT_FOUND,
        ),
        (
            "todo_item_not_uuid",
            "banana",
            "INVALID_UUID",
            StatusCode::BAD_REQUEST,
        ),
    ];
    for test_case in test_cases {
        let delete_response = test_app.delete_todo_item(test_case.1, test_case.2).await;
        assert_eq!(delete_response.status(), test_case.3, "{}", test_case.0);
    }
}
