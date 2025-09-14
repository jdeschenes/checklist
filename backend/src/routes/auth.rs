use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use eyre::Result;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use urlencoding;

use crate::{domain::User, repos::UserRepository, AppState};

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
    let token_result = client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(async_http_client)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Get user info from Google
    let google_oauth = match &state.auth {
        crate::configuration::AuthSettings::GoogleOAuth { google_oauth, .. } => google_oauth,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user_info = get_google_user_info(
        token_result.access_token().secret(),
        &google_oauth.userinfo_url,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !user_info.verified_email {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Find user (do not create if not exists)
    let user_repo = UserRepository::new(state.pool.clone());
    let user = match user_repo.find_by_email(&user_info.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // User not found - return 403 Forbidden
            return Err(StatusCode::FORBIDDEN);
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Generate JWT token
    let token = state
        .jwt_service
        .generate_token(user.user_id, &user.email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Redirect to frontend callback with token and user data as query parameters
    let frontend_callback_url = format!(
        "http://localhost:5173/auth/callback?token={}&user_id={}&email={}",
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

    let user_info: GoogleUserInfo = response.json().await?;
    Ok(user_info)
}
