use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use eyre::Result;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, RequestTokenError, Scope, TokenResponse, TokenUrl,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

use crate::{domain::User, repos::find_by_email, AppState};

#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    code: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    user: User,
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    email: String,
    verified_email: bool,
}

pub async fn google_login(State(state): State<AppState>) -> impl IntoResponse {
    let client = create_oauth_client(&state);

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    Redirect::to(auth_url.as_str())
}

pub async fn google_callback(
    Query(params): Query<AuthCallbackQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let client = create_oauth_client(&state);

    // Exchange authorization code for access token
    let token_result = match client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(async_http_client)
        .await
    {
        Ok(token_result) => token_result,
        Err(err) => {
            match &err {
                RequestTokenError::ServerResponse(server_error) => {
                    error!(
                        error = %server_error.error(),
                        error_description = ?server_error.error_description(),
                        error_uri = ?server_error.error_uri(),
                        "OAuth token exchange failed with server response"
                    );
                }
                RequestTokenError::Request(request_error) => {
                    error!(
                        error = %request_error,
                        "OAuth token exchange request failed"
                    );
                }
                RequestTokenError::Parse(parse_error, body) => {
                    error!(
                        error = %parse_error,
                        body = %String::from_utf8_lossy(body),
                        "OAuth token exchange failed to parse response"
                    );
                }
                RequestTokenError::Other(message) => {
                    error!(
                        error = %message,
                        "OAuth token exchange failed with unexpected error"
                    );
                }
            }
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Get user info from Google
    let google_oauth = match &state.auth {
        crate::configuration::AuthSettings::GoogleOAuth { google_oauth, .. } => google_oauth,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user_info = match get_google_user_info(
        token_result.access_token().secret(),
        &google_oauth.userinfo_url,
    )
    .await
    {
        Ok(user_info) => user_info,
        Err(err) => {
            error!(error = %err, "Failed to fetch Google user info");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !user_info.verified_email {
        warn!(email = %user_info.email, "Google user email not verified");
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut transaction = state.tx_state.transaction().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Find user (do not create if not exists)
    let user = match find_by_email(&mut transaction, &user_info.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // User not found - return 403 Forbidden
            warn!(email = %user_info.email, "OAuth user not authorized");
            return Err(StatusCode::FORBIDDEN);
        }
        Err(err) => {
            error!(error = %err, "Failed to lookup OAuth user");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Generate JWT token
    let token = match state.jwt_service.generate_token(user.user_id, &user.email) {
        Ok(token) => token,
        Err(err) => {
            error!(error = %err, "Failed to generate JWT token");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Redirect to frontend callback with token and user data as query parameters
    let frontend_callback_base = &google_oauth.frontend_callback_url;
    let query_sep = if frontend_callback_base.contains('?') {
        "&"
    } else {
        "?"
    };
    let frontend_callback_url = format!(
        "{}{}token={}&user_id={}&email={}",
        frontend_callback_base,
        query_sep,
        urlencoding::encode(&token),
        urlencoding::encode(&user.user_id.to_string()),
        urlencoding::encode(&user.email)
    );

    Ok(Redirect::to(&frontend_callback_url))
}

fn create_oauth_client(state: &AppState) -> BasicClient {
    let google_oauth = match &state.auth {
        crate::configuration::AuthSettings::GoogleOAuth { google_oauth, .. } => google_oauth,
        _ => panic!("Google OAuth config should be available when using Google OAuth auth"),
    };

    let google_client_id = ClientId::new(google_oauth.client_id.clone());
    let google_client_secret =
        ClientSecret::new(google_oauth.client_secret.expose_secret().to_string());
    let auth_url =
        AuthUrl::new(google_oauth.auth_url.clone()).expect("Invalid authorization endpoint URL");
    let token_url =
        TokenUrl::new(google_oauth.token_url.clone()).expect("Invalid token endpoint URL");

    BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(google_oauth.redirect_uri.clone()).expect("Invalid redirect URL"),
    )
}

async fn get_google_user_info(access_token: &str, userinfo_url: &str) -> Result<GoogleUserInfo> {
    let client = reqwest::Client::new();
    let response = client
        .get(userinfo_url)
        .bearer_auth(access_token)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        return Err(eyre::eyre!(
            "Google userinfo request failed with status {}",
            status
        ));
    }

    let user_info: GoogleUserInfo = response.json().await?;
    Ok(user_info)
}
