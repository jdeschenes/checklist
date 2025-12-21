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

    // Verify a todo item was created immediately (advance generation)
    let items_after = app.list_todo_items(todo_name).await;
    assert_response(&items_after, StatusCode::OK);
    let items_after_json: serde_json::Value = items_after.json().await.unwrap();
    let todo_items = items_after_json["items"].as_array().unwrap();
    assert_eq!(todo_items.len(), 1);
    assert_eq!(todo_items[0]["title"].as_str().unwrap(), "Daily task");

    // Verify the due date is 7 days from now
    let due_date_str = todo_items[0]["due_date"].as_str().unwrap();
    let due_date = time::Date::parse(
        due_date_str,
        &time::format_description::well_known::Iso8601::DATE,
    )
    .unwrap();
    let expected_due_date = time::OffsetDateTime::now_utc().date() + time::Duration::days(7);
    assert_eq!(due_date, expected_due_date);
}

#[tokio::test]
async fn recurring_template_creates_todo_one_week_in_advance() {
    let app = spawn_app().await;
    let todo_name = "recurring_advance";

    create_todo(&app, todo_name).await;

    // Create a template with start date far in the future (beyond 7-day window)
    // so no immediate todo creation happens
    let far_future_start = time::OffsetDateTime::now_utc().date() + time::Duration::days(20);
    let payload = json!({
        "title": "Weekly advance task",
        "recurrence_interval": {
            "days": 1
        },
        "start_date": far_future_start.to_string(),
        "end_date": null
    });
    let response = app.post_recurring_template(todo_name, &payload).await;
    assert_response(&response, StatusCode::OK);

    // Verify no todo items exist initially (start date is too far in future)
    let initial_items = app.list_todo_items(todo_name).await;
    assert_response(&initial_items, StatusCode::OK);
    let initial_items_json: serde_json::Value = initial_items.json().await.unwrap();
    let initial_todo_items = initial_items_json["items"].as_array().unwrap();
    assert_eq!(initial_todo_items.len(), 0);

    // Simulate time passing: update the template to have start_date = today
    // This makes the template eligible for the scheduler's 7-day advance processing
    let template_response: serde_json::Value = response.json().await.unwrap();
    let template_id_str = template_response["template_id"].as_str().unwrap();
    let template_id = uuid::Uuid::parse_str(template_id_str).unwrap();

    let current_date = time::OffsetDateTime::now_utc().date();

    // Update the database to simulate the template becoming active today
    sqlx::query!(
        "UPDATE recurring_template SET start_date = $1 WHERE template_id = $2",
        current_date,
        template_id
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    // Process recurring templates - this tests the scheduler's 7-day advance logic
    app.process_recurring_templates(std::time::Duration::from_secs(7 * 24 * 60 * 60))
        .await
        .unwrap();

    // Verify a todo item was created
    let items_after = app.list_todo_items(todo_name).await;
    assert_response(&items_after, StatusCode::OK);
    let items_after_json: serde_json::Value = items_after.json().await.unwrap();
    let todo_items = items_after_json["items"].as_array().unwrap();
    assert_eq!(todo_items.len(), 1);
    assert_eq!(
        todo_items[0]["title"].as_str().unwrap(),
        "Weekly advance task"
    );

    // Parse the due date and verify it's 7 days from now
    let due_date_str = todo_items[0]["due_date"].as_str().unwrap();
    let due_date = time::Date::parse(
        due_date_str,
        &time::format_description::well_known::Iso8601::DATE,
    )
    .unwrap();
    let current_date = time::OffsetDateTime::now_utc().date();
    let expected_due_date = current_date + time::Duration::days(7);
    assert_eq!(due_date, expected_due_date);

    // Process again - should not create another todo item because one already exists
    app.process_recurring_templates(std::time::Duration::from_secs(7 * 24 * 60 * 60))
        .await
        .unwrap();

    let items_after_second = app.list_todo_items(todo_name).await;
    assert_response(&items_after_second, StatusCode::OK);
    let items_after_second_json: serde_json::Value = items_after_second.json().await.unwrap();
    let todo_items_second = items_after_second_json["items"].as_array().unwrap();
    assert_eq!(todo_items_second.len(), 1); // Still only one item
}

#[tokio::test]
async fn recurring_template_creates_todo_with_proper_relationship() {
    let app = spawn_app().await;
    let todo_name = "recurr_rel";

    create_todo(&app, todo_name).await;

    // Create a daily recurring template
    let payload = json!({
        "title": "Relationship test task",
        "recurrence_interval": {
            "days": 1
        },
        "start_date": "2020-01-01",
        "end_date": null
    });
    let response = app.post_recurring_template(todo_name, &payload).await;
    assert_response(&response, StatusCode::OK);
    let template_response: serde_json::Value = response.json().await.unwrap();
    let template_id = template_response["template_id"].as_str().unwrap();

    // Process recurring templates - this should create a todo item linked to the template
    app.process_recurring_templates(std::time::Duration::from_secs(7 * 24 * 60 * 60))
        .await
        .unwrap();

    // Verify a todo item was created
    let items_after = app.list_todo_items(todo_name).await;
    assert_response(&items_after, StatusCode::OK);
    let items_after_json: serde_json::Value = items_after.json().await.unwrap();
    let todo_items = items_after_json["items"].as_array().unwrap();
    assert_eq!(todo_items.len(), 1);
    assert_eq!(
        todo_items[0]["title"].as_str().unwrap(),
        "Relationship test task"
    );

    // Verify the foreign key relationship by checking the database directly
    let todo_item_id = todo_items[0]["todo_item_id"].as_str().unwrap();
    let todo_item_uuid = uuid::Uuid::parse_str(todo_item_id).unwrap();

    // Check that the todo item has the correct recurring_template_id
    let query_result = sqlx::query!(
        "SELECT recurring_template_id FROM todo_item WHERE todo_item_id = $1",
        todo_item_uuid
    )
    .fetch_one(&app.db_pool)
    .await
    .unwrap();

    assert!(query_result.recurring_template_id.is_some());
    assert_eq!(
        query_result.recurring_template_id.unwrap().to_string(),
        template_id
    );

    // Create a second regular todo item (not from template) in the same todo
    let regular_payload = json!({
        "title": "Regular task"
    });
    let regular_response = app.post_todo_item(todo_name, &regular_payload).await;
    assert_response(&regular_response, StatusCode::OK);

    // Verify regular todo item doesn't have a recurring_template_id
    let regular_response_json: serde_json::Value = regular_response.json().await.unwrap();
    let regular_todo_item_id = regular_response_json["todo_item_id"].as_str().unwrap();
    let regular_todo_item_uuid = uuid::Uuid::parse_str(regular_todo_item_id).unwrap();

    let regular_query_result = sqlx::query!(
        "SELECT recurring_template_id FROM todo_item WHERE todo_item_id = $1",
        regular_todo_item_uuid
    )
    .fetch_one(&app.db_pool)
    .await
    .unwrap();

    assert!(regular_query_result.recurring_template_id.is_none());
}

#[tokio::test]
async fn recurring_template_with_future_start_date_creates_no_todos() {
    let app = spawn_app().await;
    let todo_name = "future_start";

    create_todo(&app, todo_name).await;

    // Create a template with start date far in the future
    let future_date = time::OffsetDateTime::now_utc().date() + time::Duration::days(30);
    let payload = json!({
        "title": "Future task",
        "recurrence_interval": {
            "days": 1
        },
        "start_date": future_date.to_string(),
        "end_date": null
    });
    let response = app.post_recurring_template(todo_name, &payload).await;
    assert_response(&response, StatusCode::OK);

    // Verify no todo items were created (start date is too far in future)
    let items_after = app.list_todo_items(todo_name).await;
    assert_response(&items_after, StatusCode::OK);
    let items_after_json: serde_json::Value = items_after.json().await.unwrap();
    let todo_items = items_after_json["items"].as_array().unwrap();
    assert_eq!(todo_items.len(), 0);
}

#[tokio::test]
async fn updating_recurring_template_creates_todo_when_necessary() {
    let app = spawn_app().await;
    let todo_name = "update_test";

    create_todo(&app, todo_name).await;

    // Create a template with start date far in the future (no immediate todo creation)
    let far_future_start = time::OffsetDateTime::now_utc().date() + time::Duration::days(20);
    let payload = json!({
        "title": "Update test task",
        "recurrence_interval": {
            "days": 1
        },
        "start_date": far_future_start.to_string(),
        "end_date": null
    });
    let response = app.post_recurring_template(todo_name, &payload).await;
    assert_response(&response, StatusCode::OK);
    let template_response: serde_json::Value = response.json().await.unwrap();
    let template_id = template_response["template_id"].as_str().unwrap();

    // Verify no todo items exist initially
    let initial_items = app.list_todo_items(todo_name).await;
    assert_response(&initial_items, StatusCode::OK);
    let initial_items_json: serde_json::Value = initial_items.json().await.unwrap();
    let initial_todo_items = initial_items_json["items"].as_array().unwrap();
    assert_eq!(initial_todo_items.len(), 0);

    // Update the template to have start_date = today (this should trigger todo creation)
    let current_date = time::OffsetDateTime::now_utc().date();
    let update_payload = json!({
        "title": "Updated task title",
        "recurrence_interval": {
            "days": 1
        },
        "start_date": current_date.to_string(),
        "end_date": null,
        "is_active": true
    });
    let update_response = app
        .update_recurring_template(todo_name, template_id, &update_payload)
        .await;
    assert_response(&update_response, StatusCode::OK);

    // Verify a todo item was created during the update
    let items_after_update = app.list_todo_items(todo_name).await;
    assert_response(&items_after_update, StatusCode::OK);
    let items_after_update_json: serde_json::Value = items_after_update.json().await.unwrap();
    let todo_items_after_update = items_after_update_json["items"].as_array().unwrap();
    assert_eq!(todo_items_after_update.len(), 1);
    assert_eq!(
        todo_items_after_update[0]["title"].as_str().unwrap(),
        "Updated task title"
    );

    // Verify the due date is 7 days from now
    let due_date_str = todo_items_after_update[0]["due_date"].as_str().unwrap();
    let due_date = time::Date::parse(
        due_date_str,
        &time::format_description::well_known::Iso8601::DATE,
    )
    .unwrap();
    let expected_due_date = current_date + time::Duration::days(7);
    assert_eq!(due_date, expected_due_date);

    // Verify the todo item is properly linked to the template
    let todo_item_id = todo_items_after_update[0]["todo_item_id"].as_str().unwrap();
    let todo_item_uuid = uuid::Uuid::parse_str(todo_item_id).unwrap();

    let query_result = sqlx::query!(
        "SELECT recurring_template_id FROM todo_item WHERE todo_item_id = $1",
        todo_item_uuid
    )
    .fetch_one(&app.db_pool)
    .await
    .unwrap();

    assert!(query_result.recurring_template_id.is_some());
    assert_eq!(
        query_result.recurring_template_id.unwrap().to_string(),
        template_id
    );
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
            .post(format!("{}/todo/{}/recurring", address, todo_name))
            .header("Authorization", app.get_auth_header());
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
    let response = app
        .post_todo(&json!({"name": todo_name, "visibility": "private"}))
        .await;

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
