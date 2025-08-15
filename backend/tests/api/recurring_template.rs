use reqwest::StatusCode;
use serde_json::json;
use serde_json::Value as JsonValue;

use crate::helpers::assert_response;
use crate::helpers::{spawn_app, TestApp};

#[tokio::test]
async fn create_recurring_template_works() {
    let app = spawn_app().await;
    let todo_name = "recurring1";

    create_todo(&app, todo_name).await;

    // Verify no todo items exist initially
    let initial_items = app.list_todo_items(todo_name).await;
    assert_response(&initial_items, StatusCode::OK);
    let initial_items_json: serde_json::Value = initial_items.json().await.unwrap();
    assert_eq!(initial_items_json["items"].as_array().unwrap().len(), 0);

    let payload = json!({
        "title": "Daily task",
        "recurrence_interval": {
            "days": 1
        },
        "start_date": "2020-01-01",
        "end_date": null
    });
    let response = app.post_recurring_template(todo_name, &payload).await;
    assert_response(&response, StatusCode::OK);
    let expected: serde_json::Value = response.json().await.unwrap();
    app.golden.check_diff_json("create_template", &expected);

    // Verify a todo item was created (should happen automatically now)
    let items_after = app.list_todo_items(todo_name).await;
    assert_response(&items_after, StatusCode::OK);
    let items_after_json: serde_json::Value = items_after.json().await.unwrap();
    let todo_items = items_after_json["items"].as_array().unwrap();
    assert_eq!(todo_items.len(), 1);
    assert_eq!(todo_items[0]["title"].as_str().unwrap(), "Daily task");
}

#[tokio::test]
async fn create_recurring_template_fails() {
    let app = spawn_app().await;
    let address = app.address.clone();
    let todo_name = "recurring2";

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
            json: Some(json!({
                "title": "Invalid task",
                "recurrence_interval": "invalid period",
                "start_date": "2025-01-01",
                "end_date": null
            })),
            expected_status_code: StatusCode::UNPROCESSABLE_ENTITY,
        },
    ];
    create_todo(&app, todo_name).await;

    for case in cases {
        let mut req = app
            .client
            .post(format!("{}/todo/{}/recurring", address, todo_name));
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
async fn list_recurring_templates_works() {
    let app = spawn_app().await;
    let todo_name = "recurring3";

    // First create a todo
    create_todo(&app, todo_name).await;

    // Create two recurring templates
    let c1 = app
        .post_recurring_template(
            todo_name,
            &json!({
                "title": "Daily task",
                "recurrence_interval": {"days": 2},
                "start_date": "2025-01-01",
                "end_date": null
            }),
        )
        .await;
    assert_response(&c1, StatusCode::OK);

    let c2 = app
        .post_recurring_template(
            todo_name,
            &json!({
                "title": "Weekly task",
                "recurrence_interval": {"days": 1},
                "start_date": "2025-01-01",
                "end_date": null
            }),
        )
        .await;
    assert_response(&c2, StatusCode::OK);

    // List recurring templates
    let response = app.list_recurring_templates(todo_name).await;
    assert_response(&response, StatusCode::OK);
    let expected: serde_json::Value = response.json().await.unwrap();
    app.golden.check_diff_json("list_templates", &expected);
}

#[tokio::test]
async fn get_recurring_template_works() {
    let app = spawn_app().await;
    let todo_name = "recurring4";

    // First create a todo
    create_todo(&app, todo_name).await;

    // Create a recurring template
    let create_response = app
        .post_recurring_template(
            todo_name,
            &json!({
                "title": "Daily task",
                "recurrence_interval": {"days": 1},
                "start_date": "2025-01-01",
                "end_date": null
            }),
        )
        .await;
    assert_response(&create_response, StatusCode::OK);

    let create_body: serde_json::Value = create_response.json().await.unwrap();
    let template_id = create_body["template_id"].as_str().unwrap();

    // Get the recurring template
    let response = app.get_recurring_template(todo_name, template_id).await;
    assert_response(&response, StatusCode::OK);

    let expected: serde_json::Value = response.json().await.unwrap();
    app.golden.check_diff_json("get_template", &expected);
}

#[tokio::test]
async fn update_recurring_template_works() {
    let app = spawn_app().await;
    let todo_name = "recurring5";

    // First create a todo
    create_todo(&app, todo_name).await;

    // Create a recurring template
    let create_response = app
        .post_recurring_template(
            todo_name,
            &json!({
                "title": "Daily task",
                "recurrence_interval": {
                    "days": 1
                },
                "start_date": "2025-01-01",
                "end_date": null
            }),
        )
        .await;

    let create_body: serde_json::Value = create_response.json().await.unwrap();
    let template_id = create_body["template_id"].as_str().unwrap();

    // Update the recurring template
    let response = app
        .update_recurring_template(
            todo_name,
            template_id,
            &json!({
                "title": "Updated daily task",
                "recurrence_interval": {
                    "days": 2
                },
                "start_date": "2025-01-02",
                "is_active": true,
                "end_date": null
            }),
        )
        .await;
    assert_response(&response, StatusCode::OK);
    let expected: serde_json::Value = response.json().await.unwrap();
    app.golden.check_diff_json("update_template", &expected);
}

#[tokio::test]
async fn delete_recurring_template_works() {
    let app = spawn_app().await;
    let todo_name = "recurring6";

    // First create a todo
    create_todo(&app, todo_name).await;

    // Create a recurring template
    let create_response = app
        .post_recurring_template(
            todo_name,
            &json!({
                "title": "Daily task",
                "recurrence_interval": {
                    "days": 1
                },
                "start_date": "2025-01-01",
                "end_date": null
            }),
        )
        .await;

    let create_body: serde_json::Value = create_response.json().await.unwrap();
    let template_id = create_body["template_id"].as_str().unwrap();

    // Delete the recurring template
    let response = app.delete_recurring_template(todo_name, template_id).await;
    assert_response(&response, StatusCode::NO_CONTENT);

    // Verify it's deleted by trying to get it
    let get_response = app.get_recurring_template(todo_name, template_id).await;
    assert_response(&get_response, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn recurring_template_fails_nonexistent_todo() {
    let app = spawn_app().await;

    let response = app
        .post_recurring_template(
            "nonexistent",
            &json!({
                "title": "Daily task",
                "recurrence_interval": {
                    "days": 1
                },
                "start_date": "2025-01-01",
                "end_date": null
            }),
        )
        .await;

    assert_response(&response, StatusCode::NOT_FOUND);
}

async fn create_todo(app: &TestApp, todo_name: &str) {
    let response = app.post_todo(&json!({"name": todo_name})).await;

    if response.status() != 200 {
        eprintln!(
            "Failed to create todo '{}': status = {}",
            todo_name,
            response.status()
        );
        if let Ok(body) = response.text().await {
            eprintln!("Response body: {}", body);
        }
        panic!("Todo creation failed");
    }
}
