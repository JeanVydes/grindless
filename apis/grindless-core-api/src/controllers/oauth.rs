use crate::{env::Enviroment, services::DEFAULT_STARTER_CREDITS, state::APIStateWrapper, util::get_claims_from_header};
use actix_web::{web, HttpRequest, Responder, Result};
use chrono::{Duration, Utc};
use grindless_core::{
    entities::{
        account::Model,
        account_ops::{AccountMutationCore, AccountQueryCore}, billing,
    },
    response::{
        errors::Errors,
        response::{build_err, build_ok, ResponseBuilderError, ResponseBuilderOk},
    },
    util::{
        jwt::{self, JWTClaims, JWTTokenType},
        random_int,
    },
    Timestamp, ID,
};
use jsonwebtoken::Algorithm;
use log::{debug, error};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

pub const GOOGLE_GET_TOKENS_URL: &str = "https://oauth2.googleapis.com/token";
pub const GOOGLE_GET_PROFILE_URL: &str = "https://www.googleapis.com/oauth2/v1/userinfo";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessGoogleForm {
    pub code: Option<String>,
}

pub async fn access_google_controller(
    state: APIStateWrapper,
    data: web::Form<AccessGoogleForm>,
) -> Result<impl Responder> {
    let code = match &data.code {
        Some(u) => u,
        None => {
            return Ok(build_err(ResponseBuilderError {
                message: "no code".to_string(),
                errors: vec![],
            }))
        }
    };

    // Get Google user tokens
    let tokens = match get_google_user_tokens(code.to_owned(), state.env.clone()).await {
        Ok(t) => t,
        Err(err) => {
            return Ok(build_err(ResponseBuilderError {
                message: err.to_string(),
                errors: vec![],
            }))
        }
    };

    if tokens.access_token.is_empty() {
        return Ok(build_err(ResponseBuilderError {
            message: "Invalid access token".to_string(),
            errors: vec![],
        }));
    }

    let profile = match get_google_user_profile(tokens.access_token).await {
        Ok(p) => p,
        Err(_) => {
            return Ok(build_err(ResponseBuilderError {
                message: "Error getting Google profile".to_string(),
                errors: vec![],
            }))
        }
    };

    error!("Google profile: {:?}", profile);

    if profile.id.is_empty() {
        debug!("Invalid Google profile");

        return Ok(build_err(ResponseBuilderError {
            message: "Invalid email".to_string(),
            errors: vec![],
        }));
    }

    // Check if account already exists
    let mut account = match AccountQueryCore::get_account_by_google_id(
        &state.databases.postgres_conn,
        &profile.id,
    )
    .await
    {
        Ok(a) => a,
        Err(err) => {
            error!("Error getting account {:?}", err);
            return Ok(build_err(ResponseBuilderError {
                message: "Error getting account".to_string(),
                errors: vec![],
            }))
        }
    };

    let now_unix = Utc::now().timestamp() as Timestamp;
    if account.is_none() {
        debug!("Account does not exist, creating new account");
        let billing = billing::Model {
            id: random_int() as ID,
            account_id: 0,
            credits: DEFAULT_STARTER_CREDITS,
            total_spent_usd: 0.0,
            created_at: now_unix,
            updated_at: now_unix,
        };

        let new_account = Model {
            id: random_int() as ID,
            google_id: profile.id,
            email: profile.email,
            name: profile.name,
            avatar: profile.picture,
            flags: vec![],
            billing_id: 0,
            created_at: now_unix,
            updated_at: now_unix,
            deleted: false,
            deletion_requested_at: None,
            deletion_reason: None,
        };

        match AccountMutationCore::create_account(
            &state.databases.postgres_conn,
            new_account.clone(),
            billing,
        )
        .await
        {
            Ok(_) => (),
            Err(_) => {
                return Ok(build_err(ResponseBuilderError {
                    message: "Error creating account".to_string(),
                    errors: vec![],
                }))
            }
        };

        account = Some(new_account);
    }

    let account = account.unwrap();
    let token = match jwt::new_token(
        &state.pems.tokens_private,
        Algorithm::RS256,
        JWTClaims {
            r#type: JWTTokenType::Access,
            sub: account.id,
            provider_id: account.google_id,
            exp: (Utc::now() + Duration::days(30)).timestamp() as i64,
            iss: Some(now_unix),
            max_requests_per_hour: 512,
        },
    ) {
        Ok(t) => t,
        Err(_) => {
            return Ok(build_err(ResponseBuilderError {
                message: "Error creating token".to_string(),
                errors: vec![],
            }));
        }
    };

    Ok(build_ok(ResponseBuilderOk {
        message: Some("Successfully logged in".to_string()),
        data: Some(token),
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckTokenForm {
    pub token: Option<String>,
}

pub async fn validate_token_controller(
    req: HttpRequest,
    state: APIStateWrapper,
) -> Result<impl Responder> {
    let claims = match get_claims_from_header(req.headers().get("Authorization"), state.pems.tokens_public.clone()) {
        Ok(c) => c,
        Err(e) => {
            return Ok(e);
        }
    };

    Ok(build_ok(ResponseBuilderOk {
        message: Some("Token is valid".to_string()),
        data: Some(claims),
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGoogleTokens {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub grant_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
}

pub async fn get_google_user_tokens(
    code: String,
    env: Enviroment,
) -> Result<OAuthResponse, Errors> {
    let mut redirect_uri = env.oauth.google.client_redirects[1].clone();
    if env.production {
        redirect_uri = env.oauth.google.client_redirects[0].clone();
    }

    let params = [
        ("grant_type", "authorization_code"),
        (
            "redirect_uri",
            redirect_uri.as_str(),
        ),
        ("client_id", env.oauth.google.client_id.as_str()),
        ("code", code.as_str()),
        ("client_secret", env.oauth.google.client_secret.as_str()),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post(GOOGLE_GET_TOKENS_URL)
        .form(&params)
        .send()
        .await.map_err(|_| Errors::BadRequest)?;

    if response.status().is_success() {
        let oauth_response = response
            .json::<OAuthResponse>()
            .await
            .map_err(|_| Errors::BadRequest)?;
        Ok(oauth_response)
    } else {
        Err(Errors::BadRequest)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

pub async fn get_google_user_profile(access_token: String) -> Result<GoogleUserResult, Errors> {
    let client = Client::new();
    let mut url = Url::parse(GOOGLE_GET_PROFILE_URL).unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", &access_token);

    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await.map_err(|_| Errors::BadRequest)?;

    if response.status().is_success() {
        let user_info = response
            .json::<GoogleUserResult>()
            .await.map_err(|_| Errors::BadRequest)?;

        Ok(user_info)
    } else {
        Err(Errors::BadRequest)
    }
}
