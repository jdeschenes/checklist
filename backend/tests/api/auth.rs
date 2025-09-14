use reqwest::StatusCode;
use serde_json::Value as JsonValue;

use crate::helpers::{spawn_app, TestApp};

/// Test that routes return 401 when no authorization header is present
#[tokio::test]
async fn routes_return_401_without_authorization_header() {
    let test_app = spawn_app().await;

    let endpoints = vec![
        ("GET", format!("{}/todo", test_app.address)),
        ("POST", format!("{}/todo", test_app.address)),
        ("GET", format!("{}/todo/test", test_app.address)),
        ("PUT", format!("{}/todo/test", test_app.address)),
        ("DELETE", format!("{}/todo/test", test_app.address)),
        ("GET", format!("{}/todo/test/item", test_app.address)),
        ("POST", format!("{}/todo/test/item", test_app.address)),
        ("GET", format!("{}/todo/test/item/123", test_app.address)),
        ("PUT", format!("{}/todo/test/item/123", test_app.address)),
        (
            "POST",
            format!("{}/todo/test/item/123/complete", test_app.address),
        ),
        ("DELETE", format!("{}/todo/test/item/123", test_app.address)),
        ("GET", format!("{}/todo/test/recurring", test_app.address)),
        ("POST", format!("{}/todo/test/recurring", test_app.address)),
        (
            "GET",
            format!("{}/todo/test/recurring/123", test_app.address),
        ),
        (
            "PUT",
            format!("{}/todo/test/recurring/123", test_app.address),
        ),
        (
            "DELETE",
            format!("{}/todo/test/recurring/123", test_app.address),
        ),
    ];

    for (method, url) in endpoints {
        let response = match method {
            "GET" => test_app.client.get(&url).send().await,
            "POST" => {
                test_app
                    .client
                    .post(&url)
                    .json(&JsonValue::Null)
                    .send()
                    .await
            }
            "PUT" => {
                test_app
                    .client
                    .put(&url)
                    .json(&JsonValue::Null)
                    .send()
                    .await
            }
            "DELETE" => test_app.client.delete(&url).send().await,
            _ => panic!("Unsupported method: {}", method),
        }
        .expect("Failed to execute request");

        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "Expected 401 for {} {} without authorization header",
            method,
            url
        );
    }
}

/// Test that routes return 401 when authorization header is invalid
#[tokio::test]
async fn routes_return_401_with_invalid_authorization_header() {
    let test_app = spawn_app().await;

    let invalid_headers = vec![
        "Bearer invalid_token",
        "Bearer ",
        "Basic dGVzdDp0ZXN0", // Basic auth instead of Bearer
        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid.signature", // Invalid JWT
    ];

    for invalid_header in invalid_headers {
        // Test a few representative endpoints
        let endpoints = vec![
            ("GET", format!("{}/todo", test_app.address)),
            ("POST", format!("{}/todo", test_app.address)),
            ("GET", format!("{}/todo/test", test_app.address)),
        ];

        for (method, url) in endpoints {
            let response = match method {
                "GET" => {
                    test_app
                        .client
                        .get(&url)
                        .header("Authorization", invalid_header)
                        .send()
                        .await
                }
                "POST" => {
                    test_app
                        .client
                        .post(&url)
                        .header("Authorization", invalid_header)
                        .json(&JsonValue::Null)
                        .send()
                        .await
                }
                _ => panic!("Unsupported method: {}", method),
            }
            .expect("Failed to execute request");

            assert_eq!(
                response.status(),
                StatusCode::UNAUTHORIZED,
                "Expected 401 for {} {} with invalid authorization header: {}",
                method,
                url,
                invalid_header
            );
        }
    }
}

/// Test that routes return 403 when user does not exist
#[tokio::test]
async fn routes_return_403_when_user_does_not_exist() {
    let test_app = spawn_app().await;

    // Generate a JWT token for a user that doesn't exist in the database
    let nonexistent_user_id = uuid::Uuid::new_v4();
    let token = test_app
        .jwt_service
        .generate_token(&nonexistent_user_id.to_string(), "nonexistent@example.com")
        .expect("Failed to generate test JWT token");
    let auth_header = format!("Bearer {}", token);

    // Test a few representative endpoints
    let endpoints = vec![
        ("GET", format!("{}/todo", test_app.address)),
        ("POST", format!("{}/todo", test_app.address)),
        ("GET", format!("{}/todo/test", test_app.address)),
    ];

    for (method, url) in endpoints {
        let response = match method {
            "GET" => {
                test_app
                    .client
                    .get(&url)
                    .header("Authorization", &auth_header)
                    .send()
                    .await
            }
            "POST" => {
                test_app
                    .client
                    .post(&url)
                    .header("Authorization", &auth_header)
                    .json(&JsonValue::Null)
                    .send()
                    .await
            }
            _ => panic!("Unsupported method: {}", method),
        }
        .expect("Failed to execute request");

        assert_eq!(
            response.status(),
            StatusCode::FORBIDDEN,
            "Expected 403 for {} {} when user does not exist",
            method,
            url
        );
    }
}

impl TestApp {
    /// Helper method to make requests without authentication
    pub async fn request_without_auth(
        &self,
        method: &str,
        path: &str,
        body: Option<&JsonValue>,
    ) -> reqwest::Response {
        let url = format!("{}{}", self.address, path);
        match method {
            "GET" => self.client.get(&url).send().await,
            "POST" => {
                let mut req = self.client.post(&url);
                if let Some(json_body) = body {
                    req = req.json(json_body);
                }
                req.send().await
            }
            "PUT" => {
                let mut req = self.client.put(&url);
                if let Some(json_body) = body {
                    req = req.json(json_body);
                }
                req.send().await
            }
            "DELETE" => self.client.delete(&url).send().await,
            _ => panic!("Unsupported method: {}", method),
        }
        .expect("Failed to execute request")
    }

    /// Helper method to make requests with invalid auth
    pub async fn request_with_invalid_auth(
        &self,
        method: &str,
        path: &str,
        auth_header: &str,
        body: Option<&JsonValue>,
    ) -> reqwest::Response {
        let url = format!("{}{}", self.address, path);
        match method {
            "GET" => {
                self.client
                    .get(&url)
                    .header("Authorization", auth_header)
                    .send()
                    .await
            }
            "POST" => {
                let mut req = self.client.post(&url).header("Authorization", auth_header);
                if let Some(json_body) = body {
                    req = req.json(json_body);
                }
                req.send().await
            }
            "PUT" => {
                let mut req = self.client.put(&url).header("Authorization", auth_header);
                if let Some(json_body) = body {
                    req = req.json(json_body);
                }
                req.send().await
            }
            "DELETE" => {
                self.client
                    .delete(&url)
                    .header("Authorization", auth_header)
                    .send()
                    .await
            }
            _ => panic!("Unsupported method: {}", method),
        }
        .expect("Failed to execute request")
    }
}
